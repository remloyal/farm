# @farmfe/runtime

## 0.3.3

### Patch Changes

- limit the watched files to optimize cold start speed and fix lazy compilation issue"

## 0.3.2

### Patch Changes

- write resources to disk to optimize resources loading time

## 0.3.1

### Patch Changes

- Fix lazy compilation and partial bundling bugs

## 0.3.0

### Minor Changes

- f915a35: Support lazy compilation and partial bundling

  - remove resource pot graph to optimize the compilation speed
  - implement partial bundling algorithm
  - optimize @farmfe/cli, remove @farmfe/core from its dependencies
  - optimize plugin react to skip duplicate module building based on process.env.NODE_ENV

## 0.2.0

### Minor Changes

- e826221: Support css HMR and dynamic resource compiling and loading for dynamic import

## 0.1.0

### Minor Changes

- 036aab6: Support react HMR