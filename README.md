# ethkey

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/ethcore/ethabi.svg?branch=master
[travis-url]: https://travis-ci.org/ethcore/ethabi

Ethereum keys generator.

[Documentation](http://ethcore.github.io/ethabi/ethabi/index.html)

### Usage

```
Ethereum ABI coder.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethkey generate random
    ethkey generate prefix <prefix> <iterations>
    ethkey generate brain <seed>

Options:
    -h, --help         Display this message and exit.

Commands:
    generate           Generates new ethereum key.
    random             Random generation.
    prefix             Random generation, but address must start with a prefix
```

### Examples

```
ethkey generate brain "this is sparta"
```

```
secret:  54d9d007f5c79c3b7ccb6bd18513dd7c44792db66e57c0429d870df503e07996
public:  e27d6a9aed5ab079f55aca2c562280f157f141a0163ba54b8edef80a797d448d151851c13340d7fa7c6f9374e7c52ae5ac3af051547e68e35f17acd4d4e5a8a4
address: 6adbbba726a4f0a180c9f30193814299cdb8283a
```

--

```
ethkey generate random
```

```
secret:  7d29fab185a33e2cd955812397354c472d2b84615b645aa135ff539f6b0d70d5
public:  35f222d88b80151857a2877826d940104887376a94c1cbd2c8c7c192eb701df88a18a4ecb8b05b1466c5b3706042027b5e079fe3a3683e66d822b0e047aa3418
address: a8fa5dd30a87bb9e3288d604eb74949c515ab66e
```

--

```
ethabi generate prefix ff 1000
```

```
secret:  2075b1d9c124ea673de7273758ed6de14802a9da8a73ceb74533d7c312ff6acd
public:  48dbce4508566a05509980a5dd1335599fcdac6f9858ba67018cecb9f09b8c4066dc4c18ae2722112fd4d9ac36d626793fffffb26071dfeb0c2300df994bd173
address: fff7e25dff2aa60f61f9d98130c8646a01f31649
```
