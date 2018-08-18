Global Options
==============

```
--database
    JSON database file. Default: ssh-permit.json
```


Host
====

## new host
```
ssh-permit-a38 host urlsmash.403.io add
```

## new host, special ssh port
```
ssh-permit-a38 host 10.211.55.7:2222 add
```

## list all hosts
```
ssh-permit-a38 host list
```

## list specific host
```
ssh-permit-a38 host urlsmash.403.io list
```

## set host alias 
```
ssh-permit-a38 host urlsmash.403.io alias um
```

After this point you can use the alias or the hostname for all host related commands

## remove host alias 
```
ssh-permit-a38 host urlsmash.403.io alias
```

## remove host
```
ssh-permit-a38 host example.com:2222 remove
```


User
====

## new user
```
ssh-permit-a38 user obelix add
```

## list all users
```
ssh-permit-a38 user list
```

## list specific user
```
ssh-permit-a38 user obelix list
```

## user remove
```
ssh-permit-a38 user obelix remove
```

## grant access to host
```
ssh-permit-a38 user obelix grant urlsmash.403.io
```

## revoke access
```
ssh-permit-a38 user obelix revoke urlsmash.403.io
```

## remove user
```
ssh-permit-a38 user obelix remove
```


Group
=====

## new group
```
ssh-permit-a38 group gauls add
```

## list all  groups
```
ssh-permit-a38 group list
```

## list specific group
```
ssh-permit-a38 group gauls list
```

## add an user to group
```
ssh-permit-a38 group gauls add obelix
```

## remove an user from group
```
ssh-permit-a38 group gauls remove obelix
```

## Grant group to host
```
ssh-permit-a38 group gauls grant urlsmash.403.io
```

## Revoke group from host
```
ssh-permit-a38 group gauls revoke urlsmash.403.io
```


Sync
====

## With public key authentication
```
ssh-permit-a38 sync -y
```

## With password authentication
```
ssh-permit-a38 sync --password -y
```