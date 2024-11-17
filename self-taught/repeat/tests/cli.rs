use assert_cmd::Command;

#[test]
fn 文字列と回数を指定して繰り返す() {
    let mut cmd = Command::cargo_bin("repeat").unwrap();
    let assert = cmd.arg("大好き").arg("5").assert();
    assert.success().stdout("大好き大好き大好き大好き大好き\n");
}
