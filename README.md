# Fish setup
Add the following to your config

`alias kubectl 'kcwrap kubectl'`

Add any part matching the kube context of your dev, test, and prod contexts, as env variables on the pattern KCWRAP-{ENV}{num} where ENV = {DEV, TEST, PROD} e.g.
```
set -gx KCWRAP_DEV1 kubernetes-admin@kubernetes
set -gx KCWRAP_DEV2 default
set -gx KCWRAP_TEST1 test
set -gx KCWRAP_PROD1 prod
```
These variabldes decide the coloring of the confirmation prompt
