sudo pacman -Sy git --noconfirm
sudo pacman -Sy flint --noconfirm

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install-rust.sh
bash install-rust.sh -y
source $HOME/.cargo/env

git clone https://github.com/zhtluo/organtest.git
cd organtest
cargo bench
