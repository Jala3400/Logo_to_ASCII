use std::process::Command;

#[test]
fn tentacles_c() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/tentacles.png")
        .arg("-b")
        .arg("both")
        .arg("--verbose")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"
Characters:  .·'qpbd8
Char size: 8x16, Line gap: 0
Block size: 8x16
Image dimensions: 416x432
Number of characters: 52x27
                .                                   
            pd88888b.                 q'            
          p8888888888q               88q            
         88888'  '8888               888            
        p888''    '''8               '888.          
        ' .·p8888888p·.               8888q         
       .q8888888888888888·            88888         
     .888888888888888888888.        .d88888         
    ·88888888888888888888888b   .q88888888'         
   ·8888888888'' .p.   '888888 '888888''            
   888888888' .q8888q    888888 '88'                
  888888888  d8888888q    88888q ''      ..qqp88p.  
  88888888    88888888b  . '8888b '888888888888888b 
 '88888888     88888888b  q '88888· ''888888888' 88·
 888888888      88888888q  '  '88888b·..   ''  . p8 
 888888888.      88888888.      ''888888888888' .8  
 8888888888       88888888           '''''''' .d8'  
 88888888888.     88888888.           ppppp8888'    
8888888888888b    '88888888       .q888888888'      
888888888888888.  '88888888.   .q8888888888'        
 888888888888888q '888888888 .888888888888          
 8888888888888888. 888888888  88888888888           
 b8888888888888888 '888888888 88888888888           
 '8888888888888888. 888888888 '8888888888·          
  '888888888888888' 888888888. 88888888888          
   '88888888888888  8888888888 88888888888.         
    '888888888888' 88888888888 '88888888888         
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
fn tentacles_cnv() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/tentacles.png")
        .arg("-b")
        .arg("both")
        .arg("-n")
        .arg("-v")
        .arg("--verbose")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"
Characters:  .·'qpbd8
Char size: 8x16, Line gap: 0
Block size: 8x16
Image dimensions: 416x432
Number of characters: 52x27
            .pp888qp.                 q88           
          p8'       88.             p8' 8           
        q8'          '8            d8   8           
       .8     ppqq    8b           8b   8b          
       8   .pp8pq8qqq.8             8p   88.        
       8p888'       '888qq          '8    '8        
     .d8'                88q        .8     db       
    d8                     88.   .q88      db       
   8b             qqq       d8p88'        p8        
  8b          qp88'888bq      8q      pp88'         
 p8         p88'    '8.8b      8p  q88'     .....   
 8         d8        '8.8bp    d8pdbqpqp888'''  '88.
d8        d88q         8888p    d8q               '8
8b        8  8b         8b'8q     d8pq         p8  .
8         8   '8         88p88q     '88888bpqp888  8
8         dq   '8        d8 ''88pq            qd8 d8
b          8q   '8        8p     '88qqqqqpppq88' p8 
b           88.  8        d8       pd8'''''    p8'  
'            d8b 8p        8p   p88'         p8'    
p              88db        d8p88'          p8'      
b               d8b         88            8'        
8                d8         8b           d8         
8                 8b         8           db         
8b                db         8p          d8         
 8p               d8         d8           8         
 '8p              db          8           d8        
  '8q            q8           8p           8b       
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
fn tentacles_cn() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/tentacles.png")
        .arg("-b")
        .arg("both")
        .arg("-n")
        .arg("--verbose")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"
Characters:  .·'qpbd8
Char size: 8x16, Line gap: 0
Block size: 8x16
Image dimensions: 416x432
Number of characters: 52x27
8888888888888888888888888888888888888888888888888888
888888888888'       888888888888888888'q888888888888
8888888888'          '888888888888888  d888888888888
888888888     pqpq    88888888888888b   888888888888
88888888   .pp8888qqq.888888888888888p   88888888888
88888888p888'       '88888888888888888    '888888888
88888888'                d888888888888     888888888
888888                     d888888888      d88888888
8888b             qqq       d8888'        p888888888
888b          qp88'8888p      8q      pp888888888888
888         p88'    '8888      8p  q8888888888888888
88         d8        '888bp    d8pd88888888'''  '888
88        d88b         8888p    d8q               '8
8b        88888         8b'8q     d8pq         p8  .
8         888888.        88p88q     '88888bpqp888  8
8         d888888p       d888888pq            qd8 d8
b          8888888        8888888888pqqpqpppq88' p88
b           888888        d88888888888'''''    p8888
             d8888b        88888888'         p888888
               888b        d8888'          p88888888
b               d8b         88            8888888888
8                d8         8b           d8888888888
8                 8b         8           d8888888888
8b                db         8p          d8888888888
88p               d8         d8           8888888888
888p              db          8           d888888888
8888p            q8           8p           888888888
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
fn tentacles_cv() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./images/tentacles.png")
        .arg("-b")
        .arg("both")
        .arg("-v")
        .arg("--verbose")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"
Characters:  .·'qpbd8
Char size: 8x16, Line gap: 0
Block size: 8x16
Image dimensions: 416x432
Number of characters: 52x27
8888888888888'' .  '888888888888888888'  88888888888
8888888888' p888888b. 88888888888888' q8 88888888888
88888888' q8888888888q 888888888888' 888 88888888888
8888888' 88888'' '8888 d88888888888 '888  8888888888
8888888 p888'' '' '''8 d88888888888b 8888. 888888888
8888888 ' .·p8888888p·. '88888888888q 8888q 88888888
888888 .q8888888888888888· '888888888 88888 d8888888
8888 .888888888888888888888. 88888' .d88888'd8888888
888 ·88888888888888888888888b ' .q88888888' d8888888
88 ·8888888888'' .p. . '888888 '888888'' .q888888888
8  888888888' .q8888q 8  888888 '88' .q8888888888888
8 888888888  d8888888b 8  88888q '' '''' ..qpp88p. 8
b 88888888 .  88888888b  . '8888b '888888888888888b 
 '88888888 88. 888888888  q '88888· ''888888888' 888
 888888888 888q 88888888b  '  '88888b·..   ''  . p8 
 888888888. 888q 88888888. 8pq. ''888888888888' .8  
 8888888888 '888b 88888888 '8888bq.  '''''''' .d8' d
 88888888888. 888 88888888b 8888888' .qpppp8888' p88
8888888888888b  8 '88888888 8888' .q888888888' p8888
'88888888888888.  '88888888b 8 .q8888888888' q888888
 888888888888888· '888888888 .888888888888 p88888888
 8888888888888888. 888888888  88888888888 q888888888
 b8888888888888888 '888888888 88888888888 d888888888
 '8888888888888888. 888888888 '8888888888. 888888888
b '888888888888888' 888888888. 88888888888 888888888
8q '88888888888888  8888888888 88888888888. 88888888
88b '888888888888' 88888888888 '88888888888 '8888888
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());
}
