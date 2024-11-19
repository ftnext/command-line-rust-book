use assert_cmd::Command;

#[test]
fn positionコマンド_文字列と回数を指定して繰り返す() {
    let mut cmd = Command::cargo_bin("repeat").unwrap();
    let assert = cmd.arg("position").arg("大好き").arg("5").assert();
    assert.success().stdout("大好き大好き大好き大好き大好き\n");
}

#[test]
fn positionコマンド_文字列だけ指定した場合は3回繰り返す() {
    let mut cmd = Command::cargo_bin("repeat").unwrap();
    let assert = cmd.arg("position").arg("yaruzo-").assert();
    assert.success().stdout("yaruzo-yaruzo-yaruzo-\n");
}
