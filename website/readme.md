# PF Sandbox Website

## Production Setup (Ubuntu)

```bash
sudo apt-get install gcc libssl-dev lib-ssh2-1-dev pkg-config
newuser rubic
su rubic
cd ~
curl https://sh.rustup.rs -sSf | sh # select nightly
echo "PATH=$HOME/.cargo/bin:$PATH" >> .profile
echo "ROCKET_ENV=prod" >> .profile
echo "TERM=xterm" >> .profile
git clone https://github.com/rukai/pf_sandbox_infra
cd pf_sandbox_infra/website
cp nginx.conf /etc/nginx.conf
```

## Run

```bash
su rubic
tmux
cd ~/pf_sandbox_infra/website
cargo run --release
```
