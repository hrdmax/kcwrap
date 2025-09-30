# Fish setup
Add the following to your config

`alias kubectl 'kcwrap kubectl'`

Repeat for any supported command you wish, e.g.

`alias flux 'kcwrap flux'`

To force skip confirmation dialog and context print, for example when wanting to pipe command outputs, add an alias with `kcw_no_wrap` as the first argument to the supported command

`alias kubectl! 'kcwrap kubectl kcw_no_wrap'`

Add any part matching the kube context of your dev, test, and prod contexts, as env variables on the pattern KCWRAP-{ENV}{num} where ENV = {DEV, TEST, PROD} e.g.
```
set -gx KCWRAP_DEV1 kubernetes-admin@kubernetes
set -gx KCWRAP_DEV2 default
set -gx KCWRAP_TEST1 test
set -gx KCWRAP_PROD1 prod
```
These variabldes decide the coloring of the confirmation prompt
