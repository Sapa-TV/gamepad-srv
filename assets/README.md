# GamePadViewer

## General

// TODO

## Supported Controller types

Currently only xbox one controllers are supported. For any other controllers it would be neccessary to change the base images and positioning. This might be part of a future change.

## How to customize?

1. Create a new branch & fork this project
2. Modify & exchange images
3. Host all neccessary assets [images](./images). There are a couple free cdns out there.
4. Insert the static links to the image assets into the [git-variables.css-file](./git-variables.css).
5. For more advanced customization you can also modify the [custom.css file](./custom.css).

## How to use as GamePadViewer?
1. Go to [GamepadViewer](https://gamepadviewer.com/#generate). Small remark: This is an external link that is not under control.
2. Choose xbox one as skin as it is currently the only supported skin.
3. Generate a static link to the [custom-git.css-file](./custom-git.css) and enther this link into the field "Custom CSS URL".
4. Then you can generate a link to your own customized gamepad viewer which you can use e. g. as a browser source for a stream overlay.
