use crate::style::progress_bar::ProgressBarStyle;
use indicatif::{MultiProgress, ProgressBar};
use std::io;
use std::{path::Path, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::sync::Arc;
use tokio::sync::Semaphore;

pub async fn copy(
    source: &Path,
    destination: &Path,
    style: ProgressBarStyle,
    recursive: bool,
    concurrency: usize,
) -> io::Result<()> {
    let metadata_src = tokio::fs::metadata(source).await?;
    let metadata_dest = tokio::fs::metadata(destination).await.ok();

    if metadata_src.is_dir() {
        if !recursive {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "'{}' is a directory (not copied, use -r to copy recursively)",
                    source.display()
                ),
            ));
        }

        if let Some(dest_meta) = metadata_dest {
            if dest_meta.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("'{}' is a file, expected directory", destination.display()),
                ));
            }
        }

        return copy_directory(source, destination, style, concurrency).await;
    }

    let pb = ProgressBar::new(metadata_src.len());
    style.apply(&pb);

    if let Some(dest_meta) = metadata_dest {
        if dest_meta.is_dir() {
            let file_name = source.file_name().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "Invalid source path")
            })?;
            let dest_path = destination.join(file_name);
            return do_copy(source, &dest_path, &pb).await;
        }
    }

    do_copy(source, destination, &pb).await
}

pub async fn do_copy(source: &Path, destination: &Path, pb: &ProgressBar) -> io::Result<()> {
    let result = async {
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
        Ok(())
    }
    .await;

    match &result {
        Ok(_) => pb.finish_with_message("Copy complete"),
        Err(_) => pb.abandon_with_message("Copy failed"),
    }

    result
}

pub async fn multiple_copy(
    sources: Vec<PathBuf>,
    destination: PathBuf,
    style: ProgressBarStyle,
    concurrency: usize,
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

    let semaphore = Arc::new(Semaphore::new(concurrency));
    for source in sources {
        let dest = destination.clone();
        let mp = multi_progress.clone();

        let style_cloned = style;
        let semaphore = semaphore.clone();
        let task = tokio::spawn(async move {
            let _permit = semaphore
                .acquire()
                .await
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Semaphore closed"))?;
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

pub async fn copy_directory(
    source: &Path,
    destination: &Path,
    style: ProgressBarStyle,
    concurrency: usize,
) -> io::Result<()> {
    let multi_progress = MultiProgress::new();
    let semaphore = Arc::new(Semaphore::new(concurrency));

    do_copy_directory(
        source.to_path_buf(),
        destination.to_path_buf(),
        style,
        multi_progress,
        semaphore,
    )
    .await
}

pub async fn do_copy_directory(
    source: PathBuf,
    destination: PathBuf,
    style: ProgressBarStyle,
    multi_progress: MultiProgress,
    semaphore: Arc<Semaphore>,
) -> io::Result<()> {
    if let Err(e) = tokio::fs::create_dir_all(&destination).await {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(e);
        }
    }

    let mut entries = tokio::fs::read_dir(&source).await?;
    let mut tasks = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = destination.join(&file_name);
        let metadata = entry.metadata().await?;
        let mp = multi_progress.clone();
        let style_cloned = style;
        if metadata.is_dir() {
            Box::pin(do_copy_directory(
                path,
                dest_path,
                style_cloned,
                mp,
                semaphore.clone(),
            ))
            .await?;
        } else if metadata.is_file() {
            let len = metadata.len();
            let sem_clone = semaphore.clone();

            let task = tokio::spawn(async move {
                let permit = sem_clone
                    .acquire()
                    .await
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "Semaphore closed"))?;
                let pb = mp.add(ProgressBar::new(len));
                pb.set_message(format!("Copying {}", file_name.to_string_lossy()));
                style_cloned.apply(&pb);
                let result = do_copy(&path, &dest_path, &pb).await;
                drop(permit);
                result
            });
            tasks.push(task);
        }
    }

    let mut errors = Vec::new();

    for task in tasks {
        match task.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => errors.push(e.to_string()),
            Err(e) => errors.push(e.to_string()),
        }
    }

    if !errors.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Errors during directory copy:\n{}", errors.join("\n")),
        ));
    }

    Ok(())
}
