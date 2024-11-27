# pixelify

![input](https://github.com/user-attachments/assets/992bca58-4d1b-40f3-999a-ca244656e84d) | ![output](https://github.com/user-attachments/assets/e9580cee-4aee-42eb-a999-8df01cc3e217)

## Overview
This library is for easy image pixelation
The use is simple:

```bash
pixelify <image> [-o OUTPUT]
cargo run --bin pixelify pine-forest.png -s 20
```

## Features
- [x] [box-blur](https://en.wikipedia.org/wiki/Box_blur)
- [ ] randomize the resampling to get more interesting results
- [ ] [Lanczos_resampling](https://en.wikipedia.org/wiki/Lanczos_resampling)
