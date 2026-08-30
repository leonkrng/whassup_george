# whassup_george

`whassup_george` is an Linxu-Service that notifies you if George R. R. Martin posts about the next installment of the ' A Song of Ice and Fire' Series (TWoW could release any second). 
The service sends you an email if one predefined regex is matched in the most recent posts on Martins [Not a Blog](https://georgerrmartin.com/notablog/).


## Configuration
Configure the `config.json` file which is located in the `src/config/`-directory. 
Fill in your sender and receiver email and the URL of the smtp-relay you want to use.
You can also change the regexes that trigger the email.

## Installation

Clone and build the repository with `cargo`:

``` Bash 
git clone https://github.com/leonkrng/whassup_george.git
cd whassup_george
cargo build --release
``` 

Move the binary to the `opt` directory:

``` Bash
sudo mkdir -p /opt/whassup_george
sudo cp target/release/whassup_george /opt/whassup_george/whassup_george
```

Move the `systemd`-files to the `systemd`-directory:

``` Bash
sudo cp whassup_george.service /etc/systemd/system/
sudo cp whassup_george.timer /etc/systemd/system/
```

Set up your SMTP password:

``` Bash
echo 'your_password' < smtp-password
sudo mkdir -p /ect/whassup_george/
sudo mv smpt-password /etc/whassup_george/
sudo chown root:root /etc/whassup_george/smtp-password
sudo chmod 600 /etc/whassup_george/smtp-password
``` 

Enable the service:

``` Bash
sudo systemctl daemon-reload
sudo systemctl enable --now whassup_george.timer
```

You can check the state of your service:

``` Bash
systemctl status whassup_george.timer
systemctl list-timers
```
