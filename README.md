# chia-k8s

Chia ハーベスターのデプロイ構成。非公式。趣味用。参考程度に、自己責任でお願いします。

## 準備

1. `chia-daemonset.yaml` をベアメタルに合わせて書き換えます。(それぞれ同じ構成である前提です)
2. `ca-gen` を実行し `chia-secret.yaml` を作成します。これは秘密鍵を含むものであり公開してはいけない。
3. `kubectl` から `chia-k8s` を丸ごとデプロイする。

## 環境

- Raspberry Pi 4 model B を 5 個 + 1 個 (マスタ)
- 6TB HDD を 60 台 (ホスト毎に 12 台)
- Ubuntu Server, Containerd, Kubernetes, Flannel

で稼働してます。

## ほか

- プライベートレジストリから引く場合、作業前に `registrySecret` をデプロイしておく必要があります。
- VXLAN が正しく構成されていないとうまくいかないことがありました:
  https://gist.github.com/ydipeepo/e33e8bb19a8d9c6aaf4f7385f2eca52c
