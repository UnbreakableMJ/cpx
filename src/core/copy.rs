use super::progress_bar::ProgressBarStyle;
use indicatif::{MultiProgress, ProgressBar};
use std::io;
use std::{path::Path, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn copy(source: &Path, destination: &Path, style: ProgressBarStyle) -> io::Result<()> {
    let metadata = tokio::fs::metadata(source).await?;
    let pb = ProgressBar::new(metadata.len());
    style.apply(&pb);
    do_copy(source, destination, &pb).await
}

pub async fn do_copy(source: &Path, destination: &Path, pb: &ProgressBar) -> io::Result<()> {
    let mut src_file = tokio::fs::File::open(source).await?;
    let mut dest_file = tokio::fs::File::create(destination).await?;

    const BUFFER_SIZE: usize = 65536;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let bytes_read = src_file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        dest_file.write_all(&buffer[..bytes_read]).await?;
        pb.inc(bytes_read as u64);
    }

    pb.finish_with_message("Copy complete");
    Ok(())
}

pub async fn multiple_copy(
    sources: Vec<PathBuf>,
    destination: PathBuf,
    style: ProgressBarStyle,
) -> io::Result<()> {
    let dest_metadata = tokio::fs::metadata(&destination).await?;
    if !dest_metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Destination '{}' is not a directory", destination.display()),
        ));
    }

    let multi_progress = MultiProgress::new();

    let mut tasks = Vec::new();

    for source in sources {
        let dest = destination.clone();
        let mp = multi_progress.clone();

        let style_cloned = style.clone();

        let task = tokio::spawn(async move {
            let file_name = source.file_name().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "Invalid source path")
            })?;

            let dest_path = dest.join(file_name);

            let metadata = tokio::fs::metadata(&source).await?;
            let pb = mp.add(ProgressBar::new(metadata.len()));
            pb.set_message(format!("Copying {}", file_name.to_string_lossy()));
            style_cloned.apply(&pb);

            do_copy(&source, &dest_path, &pb).await?;

            Ok::<_, io::Error>(())
        });

        tasks.push(task);
    }

    let mut errors = Vec::new();
    for (i, task) in tasks.into_iter().enumerate() {
        match task.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => errors.push(format!("File {}: {}", i, e)),
            Err(e) => errors.push(format!("Task {}: {}", i, e)),
        }
    }

    if !errors.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Errors occurred:\n{}", errors.join("\n")),
        ));
    }

    Ok(())
}
