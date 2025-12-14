# Shell config setup
Some aliases and env variables are requried for kcwrap to function. First, add an alias for any of
the supported commands that you wish to have wrapped. To show supported commands, run `kcwrap` without arguments\
**fish** \
`alias kubectl 'kcwrap kubectl'`\
**bash**\
`alias kubectl='kcwrap kubectl''`

To force skip confirmation dialog and context print, for example when wanting to pipe command outputs, add an alias with `kcw_no_wrap` as the first argument to the supported command \
**fish**\
`alias kubectl! 'kubectl kcw_no_wrap'`\
**bash**\
`alias kubectl!='kubectl kcw_no_wrap'`

Add any part matching the kube context of your dev, test, and prod contexts, as env variables on the pattern KCWRAP-{ENV}{num} where ENV = {DEV, TEST, PROD} e.g.
**fish**\
```
set -gx KCWRAP_DEV1 kubernetes-admin@kubernetes
set -gx KCWRAP_DEV2 default
set -gx KCWRAP_TEST1 test
set -gx KCWRAP_PROD1 prod
```
**bash**\
```
export KCWRAP_DEV1=kubernetes-admin@kubernetes
```
These variabldes decide the coloring of the confirmation prompt
