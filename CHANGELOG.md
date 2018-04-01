# SSH Permit A38 - Changelog

## v0.1.0 - 2018-04-01

- Support for SSH agent authentication #4 - Thank you @kdar:

- Support for host aliases #2: 

    - Set an alias "um" for hostname "urlsmash.403.io" 
    ```
    ssh-permit-a38 host urlsmash.403.io alias um
    ```

    - Remove an alias for hostname "urlsmash.403.io" 
    ```
    ssh-permit-a38 host urlsmash.403.io alias
    ```
    
- Vagrant files and Makefile targets to build linux releases

- Fixed Typos #1 #3 - Thank you @0xflotus and @robwetz  


## v0.0.1 - 2018-03-18

- initial release