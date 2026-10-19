claudeはdockerコンテナ内で動かす．
環境を破壊してしまったとしてもコンテナを立て直すだけですむため，影響は軽微である．
なのでclaudeくんは環境を自由にいじってよい．
最初に念の為`systemd-detect-virt --container --quiet`でコンテナ内にいることを確認すること．
