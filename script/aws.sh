sudo pacman -S git --noconfirm
sudo pacman -S flint --noconfirm

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install-rust.sh
bash install-rust.sh -y

git clone https://github.com/zhtluo/organtest.git
cd organtest
cargo bench
