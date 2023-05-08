# just hash

want a simple way to hash a string in the terminal? You can do that with this cli.

## example

```shell
$ justhash -s "hello world!"

7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9
```

## installing

### nix

add the following input to your flake:

```nix
  justhash.url = "github:sempruijs/justhash";
```

and then adding it to home.packages

```nix
    home.packages = with pkgs; [
      home.packages = with pkgs; [
    ]
```