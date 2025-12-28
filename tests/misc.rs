use std::process::Command;

#[test]
#[ignore]
fn gradient_w80_a_aeou_m07() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/sources/gradient.jpg")
        .arg("--alg")
        .arg("gradient")
        .arg("-w")
        .arg("80")
        .arg("-x")
        .arg(" ")
        .arg("-a")
        .arg("aeou@")
        .arg("-v")
        .arg("--verbose")
        .arg("-m")
        .arg("0.7")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"
Characters: .·'uoaeqpbd8@
Char size: 8x16, Line gap: 0
Block size: 8x16
Original dimensions 1920x1600
Image dimensions: 640x533
Number of characters: 80x34
Unfilled space: 0x11 pixels
................................................................................
................................................................................
......................................................··························
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
'''''''''''''''''''''''''''''''''''''''''''''''''''''u'uuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuooooooooooooooooooooooooooooooooooooooooooooo
ooooooooooooooooooooooooooooooooooooooooooooaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
aaaaaaaaeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
eeeqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqppppppppppppppppppp
pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbdddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddd888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
888888888@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
[0m"#;

    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
#[ignore]
fn gradient_w80_a_aeou_m07_g() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/sources/gradient.jpg")
        .arg("--alg")
        .arg("gradient")
        .arg("-w")
        .arg("80")
        .arg("-x")
        .arg(" ")
        .arg("-a")
        .arg("aeou@")
        .arg("-v")
        .arg("--verbose")
        .arg("-m")
        .arg("0.7")
        .arg("-g")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"
Characters: .·'uoaeqpbd8@
Char size: 8x16, Line gap: 0
Block size: 8x16
Original dimensions 1920x1600
Image dimensions: 640x533
Number of characters: 80x34
Unfilled space: 0x11 pixels
................................................................................
................................................................................
......................................................··························
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
'''''''''''''''''''''''''''''''''''''''''''''''''''''u'uuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuooooooooooooooooooooooooooooooooooooooooooooo
ooooooooooooooooooooooooooooooooooooooooooooaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
aaaaaaaaeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
eeeqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqppppppppppppppppppp
pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbdddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddd888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
88888888888888888888888888888888888888888888888888888888888888888888888888888888
888888888@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
[0m"#;

    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
fn test_padding_both_and_separated() {
    let output1 = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/sources/Cross_Calatrava.png")
        .arg("--verbose")
        .arg("--pad")
        .arg("10")
        .output()
        .expect("Failed to execute command");

    let output2 = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/sources/Cross_Calatrava.png")
        .arg("--verbose")
        .arg("--padx")
        .arg("10")
        .arg("--pady")
        .arg("10")
        .output()
        .expect("Failed to execute command");

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    let stdout2 = String::from_utf8_lossy(&output2.stdout);

    assert_eq!(stdout1.trim(), stdout2.trim());
}

#[test]
fn test_padding_transparent() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/sources/Cross_Calatrava.png")
        .arg("--verbose")
        .arg("-X")
        .arg("1")
        .arg("-b")
        .arg("all")
        .arg("-v")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert_eq!(
        stdout.trim(),
        r#"
Characters:  .·'qpbd8
Char size: 8x16, Line gap: 0
Block size: 8x16
Applied padding of 1x0
Image dimensions: 482x480
Number of characters: 61x30
Unfilled space: 6x0 pixels
88888888888888888888888888888  888888888888888888888888888888
888888888888888888888888888' .. '8888888888888888888888888888
8888888888888888888888888b ·8888· d88888888888888888888888888
8888888888888888888888888b '8888' d88888888888888888888888888
88888888888888888888'    'b '88' p'    '888888888888888888888
88888888888888888' ·88888.  '88'  .88888· '888888888888888888
88888888888888888 d88b'''d888888888'''d88p 888888888888888888
88888888888888888 '88  pp  '8888b  qp  88' 888888888888888888
888888888888''''8b  'b '88p 8888 q88' q'  d8''''8888888888888
888888888' ·888p. 8b.  ·  8 8888 8  ·' .d8 .q888· '8888888888
888888888 d88b'''b  88pq    8888    pp88' d'''d88b d888888888
88888888b 888  p. '  8888   8888   8888  ' .p  888 d888888888
88888888b 'd8· '8b.   '  .. 8888 .   '   .d88 ·8b' d888888888
888' ··  '   8b.  ''      ' 8888 ''     ''  .d8'  '  ·· '8888
'' .8888888888888888888888888888888888888888888888888888.  '8
p.  '88888888888888888888888888888888888888888888888888b  .q8
888p '' .p  .8'  .p·     qp 8888 pp     ·q.  '8.  q.  ' q8888
88888888b ·88' q88    p.    8888   ..q    88p '88· d888888888
88888888b 888  8' ·  888b   8888   d888  ·  8  888 d888888888
88888888b '888ppd' q888'    8888    '888p 'bpp888' d888888888
888888888p ''8b'  d8' ·  .d 8888 b.  ·  8b  ''8'' q8888888888
88888888888ppppq88 .d' pd8' 8888 '8bq 'b. 88pppqp888888888888
88888888888888888 .88  8' .888888. '8  88. 888888888888888888
88888888888888888 '888ppp8b'8888''8ppp888' 888888888888888888
88888888888888888p ''8888'  '88'   d888'' q888888888888888888
8888888888888888888pqq...q8 d88b 8p...ppq88888888888888888888
88888888888888888888888888 ·8888· d88888888888888888888888888
8888888888888888888888888b ''88b' d88888888888888888888888888
888888888888888888888888888p    q8888888888888888888888888888
8888888888888888888888888888b  d88888888888888888888888888888
[0m"#
            .trim()
    );
}
