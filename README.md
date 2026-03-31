# 🕵️‍♂️ Ladroncito

**Ladroncito** is a tool for analyzing transaction paths from CSV files using graph-based techniques. It allows you to trace the flow between wallets and visualize how they are connected.

---

## 🚀 Features

- 🔗 Graph-based transaction analysis  
- 📄 CSV file support  
- ⚙️ Simple configuration via `config.yaml`  
- ⚡ Parallel processing with Rayon  
- 🌐 Lightweight web interface  

---

## 🧱 Tech Stack

- **Backend:** Rust (`Rocket`, `Rayon`)  
- **Frontend:** Preact + `react-hook-form`  

---

## ⚙️ Configuration

Edit the `config.yaml` file to define analysis parameters:

```yaml
# Path to accounts file (currently not used)
accounts_path: data/accounts_fake_test.csv

# Path to transactions file
transactions_path: data/transacciones_complejas.csv

# Initial wallet to analyze
initial_wallet: TL0_Node7_x92Jk

# Final wallet to stop tracking
final_wallet: TL4_Node5_wQ11

# Search depth (higher = more exhaustive)
# You will need additional vCPU cores to make full use of this feature
depth: 5
```

💡 This is a preconfigured test case, so you can run the project without changing anything.

---

## ▶️ Run

To execute the transaction analysis:

```bash
cargo run
```

---

## 🌐 Web Interface

To launch the web interface, it is recommended to use Docker:

```bash
docker compose up
```

Then open your browser at:  
👉 http://127.0.0.1:8000/

In production (for example on **cubepath.com**), the container exposes ports **80** and **443** via Caddy.

---

## ☁️ VPS Deployment (Debian cubepath.com)

Steps to deploy on a clean server such as cubepath.com; Note: you need to edit the "Caddyfile" indicating the new domain.

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg

sudo install -m 0755 -d /etc/apt/keyrings

curl -fsSL https://download.docker.com/linux/debian/gpg | \
  sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

sudo chmod a+r /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) \
  signed-by=/etc/apt/keyrings/docker.gpg] \
  https://download.docker.com/linux/debian \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt-get update

sudo apt-get install -y docker-ce docker-ce-cli containerd.io \
  docker-buildx-plugin docker-compose-plugin

git clone https://github.com/LuisErnestoZamb/ladroncito.git
cd ladroncito
vi Caddyfile
```

```bash
docker compose up -d
```

---

## 🎬 Demo

👉 https://vps23880.cubepath.net/

---

## 💡 About the Name

Once upon a time, someone stole crypto thinking they were invisible...  
But crypto isn’t like stealing gold 🪙  

It’s more like stealing a neon sign that says:  
**"I WENT THIS WAY!"** 🚨

---

## 📬 Contact

📧 luisezamb@gmail.com
