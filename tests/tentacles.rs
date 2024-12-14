use std::process::Command;

#[test]
fn tentacles_c() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--path")
        .arg("./images/tentacles.png")
        .arg("-c")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"
Characters:  .·'pbdq8
Image dimensions: 416x432
Number of characters: 52x27
                                                    
            .q88888p.                 q'            
          q8888888888p               88'            
         888888'''8888              '888            
        d888''    ''88               8888.          
        '  .q888888qp..               8888·         
       .q8888888888888888.            88888         
     .q88888888888888888888.         d88888'        
    ·88888888888888888888888p   .qq8888888'         
   ·8888888888b'  ..  ''88888p '8888888''           
  .888888888' .q8888.   '88888p '88'                
  888888888  q8888888p   '88888· 8.      ..·qqqqp.  
  88888888'  '88888888q    88888· '888888888888888p 
 '88888888     88888888p '. '88888. '8888888888' 888
 888888888      88888888q  '  '88888p.    '''' . 88 
 888888888.      88888888.      '8888888888888' .8' 
 8888888888       88888888          ''''8'''' .q8'  
q88888888888.     88888888.           ·qqqqq888'    
8888888888888p    888888888       .q888888888'      
888888888888888.  '88888888.   .q8888888888'        
'888888888888888· '888888888 .888888888888          
 8888888888888888. 888888888 '88888888888           
 88888888888888888 '88888888p 88888888888           
 '8888888888888888  888888888 '8888888888.          
  8888888888888888  888888888. 88888888888          
   888888888888888 .8888888888 88888888888.         
    '888888888888' 88888888888 '88888888888         
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
fn tentacles_ci() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--path")
        .arg("./images/tentacles.png")
        .arg("-c")
        .arg("-i")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"Characters:  .·'pbdq8
Image dimensions: 416x432
Number of characters: 52x27
             qppdqqq                  q88           
          q88'     '8p              q8'q8           
        q8'          '8            d8  d8           
       q8     qqqp    8            8p   8q          
       8    qq8qd8qq. 8             8p   8p         
       8qp888'    ''8888p.          '8    '8        
     .d88                88p        .8     dp       
    d8                     88.   .qd8      qp       
   8b                       d8q88''       q8        
  8b          qqp88888qp     d8p       qp8'         
 d8         qd8'    d8'8p     d8p  q88''     ....   
 8         q8        '8 8q     d8pd8.qqp8888'''''8b 
d8        q88p        '8888p    d8p               '8
8b        8  8q         8p88p     88qp         q8   
8         8p  88         8888qp     888888qqqq888  8
8         d8   '8        dq ''8qqp            qq8 q8
p          8q   d8        8p    '88qqqqppqqqq88' q8 
'           88.  8        d8       .d8''''''   q88  
             d8q 8p        8p   qp8'         q8'    
               88qp        d8.d8'          q8'      
p               d8p         88            q8        
8                88         8p           qp         
8                 8p        d8           qp         
8p                q8         8p          d8         
 8p               q8         d8           8         
 '8p              8p          8           d8        
  '8p            q8           8p           8p       
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
fn tentacles_civ() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--path")
        .arg("./images/tentacles.png")
        .arg("-c")
        .arg("-i")
        .arg("-v")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"Characters:  .·'pbdq8
Image dimensions: 416x432
Number of characters: 52x27
8888888888888888888888888888888888888888888888888888
8888888888888'     '888888888888888888'q888888888888
8888888888'          '888888888888888  d888888888888
888888888     qqqp    88888888888888p   888888888888
88888888    qq8888qq. 888888888888888p   88888888888
88888888qp888'    ''888888888888888888    '888888888
888888888                8888888888888     d88888888
888888                     8888888888      q88888888
8888b                       d8888''       q888888888
888b          qqp88888qp     d8p       qq88888888888
888         qd8'    d888p     d8p  q8888888888888888
88         q8        '888q     d8pd888888888'''''888
88        q88p        '8888p    d8p               '8
8b        8888q         8p88p     88qp         q8   
8         888888         8888qp     888888qqqq888  8
8         d888888        d88888qqp            qq8 q8
p          8888888        888888888pqqqppqqqq88' q88
'           888888        d88888888888''''''   q8888
             d8888p        88888888'         q888888
               888p        d8888'          q88888888
p               d8p         88            q888888888
8                88         8p           q8888888888
8                 8p        d8           q8888888888
8p                q8         8p          d8888888888
88p               q8         d8           8888888888
888p              8p          8           d888888888
8888p            q8           8p           888888888
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());
}

#[test]
fn tentacles_cv(){
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--path")
        .arg("./images/tentacles.png")
        .arg("-c")
        .arg("-v")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected = r#"Characters:  .·'pbdq8
Image dimensions: 416x432
Number of characters: 52x27
8888888888888'''''''888888888888888888'  88888888888
8888888888' .q88888p. 88888888888888' q' 88888888888
888888888 q8888888888p 888888888888' 88' 88888888888
8888888' 888888'''8888 d88888888888 '888 '8888888888
8888888 d888'' '' ''88 888888888888q 8888. 888888888
8888888 '  .q888888qp.. '88888888888p 8888· 88888888
888888 .q8888888888888888. '888888888 88888 q8888888
8888 .q88888888888888888888. 88888'' d88888'q8888888
888 ·88888888888888888888888p ' .qq8888888' d8888888
88 ·8888888888b'  ..  ''88888p '8888888'' q888888888
8 .888888888' .q8888. q '88888p '88' .qq888888888888
8 888888888  q8888888p 8 '88888· 8. 88'' ..·qqqqp. 8
b 88888888'  '88888888q    88888· '888888888888888p 
 '88888888 88  88888888p '. '88888. '8888888888' 888
 888888888 888p 88888888q  '  '88888p.    '''' . 88 
 888888888. 888p 88888888. qpq. '8888888888888' .8' 
 8888888888 '888p 88888888 88888p.  ''''8'''' .q8' q
q88888888888. 888 88888888. 88888888' ·qqqqq888' .88
8888888888888p '8 888888888 8888' .q888888888' q8888
888888888888888.  '88888888. 8 .q8888888888' q888888
'888888888888888· '888888888 .888888888888 q88888888
 8888888888888888. 888888888 '88888888888 q888888888
 88888888888888888 '88888888p 88888888888 d888888888
 '8888888888888888  888888888 '8888888888. 888888888
p 8888888888888888  888888888. 88888888888 888888888
8p 888888888888888 .8888888888 88888888888. 88888888
88p '888888888888' 88888888888 '88888888888 '8888888
[0m"#;
    assert_eq!(stdout.trim(), expected.trim());

}