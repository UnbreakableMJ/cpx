# CPX vs GNU cp — cold cache benchmark

## Environment
- CPU cores: 8
- OS: Linux 6.17.0-8-generic
- Date: Sat Jan 24 16:47:20 IST 2026
- cp: cp (GNU coreutils) 9.5
- Cache mode: cold

## Dataset
- Size: 13G
- Files: 773996
- Directories: 71665
- Repositories: 11

## OpenImageIO
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/OpenImageIO /home/happy/cpx_multi_bench/dest_cpx` | 124.6 ± 1.6 | 122.6 | 127.4 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/OpenImageIO /home/happy/cpx_multi_bench/dest_cp` | 258.1 ± 2.2 | 255.3 | 260.7 | 2.07 ± 0.03 |

## chromium
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/chromium /home/happy/cpx_multi_bench/dest_cpx` | 15.892 ± 0.555 | 15.236 | 16.721 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/chromium /home/happy/cpx_multi_bench/dest_cp` | 46.194 ± 1.517 | 45.158 | 49.043 | 2.91 ± 0.14 |

## kubernetes
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/kubernetes /home/happy/cpx_multi_bench/dest_cpx` | 744.2 ± 98.8 | 656.0 | 938.2 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/kubernetes /home/happy/cpx_multi_bench/dest_cp` | 2805.0 ± 10.9 | 2793.0 | 2825.2 | 3.77 ± 0.50 |

## node
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/node /home/happy/cpx_multi_bench/dest_cpx` | 1.351 ± 0.040 | 1.290 | 1.390 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/node /home/happy/cpx_multi_bench/dest_cp` | 4.573 ± 0.039 | 4.541 | 4.645 | 3.39 ± 0.10 |

## openexr
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/openexr /home/happy/cpx_multi_bench/dest_cpx` | 273.4 ± 0.8 | 272.4 | 274.6 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/openexr /home/happy/cpx_multi_bench/dest_cp` | 285.6 ± 3.1 | 282.2 | 291.0 | 1.04 ± 0.01 |

## linux
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/linux /home/happy/cpx_multi_bench/dest_cpx` | 2.925 ± 0.032 | 2.875 | 2.965 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/linux /home/happy/cpx_multi_bench/dest_cp` | 9.314 ± 0.248 | 9.089 | 9.679 | 3.18 ± 0.09 |

## vscode
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/vscode /home/happy/cpx_multi_bench/dest_cpx` | 263.4 ± 2.0 | 260.6 | 266.7 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/vscode /home/happy/cpx_multi_bench/dest_cp` | 1084.3 ± 7.7 | 1077.2 | 1096.8 | 4.12 ± 0.04 |

## go
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/go /home/happy/cpx_multi_bench/dest_cpx` | 335.5 ± 14.3 | 321.6 | 356.3 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/go /home/happy/cpx_multi_bench/dest_cp` | 1373.0 ± 16.5 | 1357.0 | 1403.4 | 4.09 ± 0.18 |

## rust
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/rust /home/happy/cpx_multi_bench/dest_cpx` | 1.091 ± 0.079 | 1.039 | 1.247 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/rust /home/happy/cpx_multi_bench/dest_cp` | 4.553 ± 0.037 | 4.525 | 4.626 | 4.17 ± 0.30 |

## godot
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/godot /home/happy/cpx_multi_bench/dest_cpx` | 353.8 ± 4.0 | 349.1 | 359.0 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/godot /home/happy/cpx_multi_bench/dest_cp` | 549.6 ± 1.6 | 547.6 | 551.3 | 1.55 ± 0.02 |

## tensorflow
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos/tensorflow /home/happy/cpx_multi_bench/dest_cpx` | 736.1 ± 31.4 | 689.8 | 757.8 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos/tensorflow /home/happy/cpx_multi_bench/dest_cp` | 2846.2 ± 19.5 | 2823.4 | 2877.2 | 3.87 ± 0.17 |

## Full Dataset
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `/home/happy/cpx/cpx -r -j=16 /home/happy/cpx_multi_bench/repos /home/happy/cpx_multi_bench/dest_cpx` | 28.721 ± 1.461 | 26.966 | 30.570 | 1.00 |
| `cp -r /home/happy/cpx_multi_bench/repos /home/happy/cpx_multi_bench/dest_cp` | 80.563 ± 2.785 | 78.578 | 84.316 | 2.81 ± 0.17 |
