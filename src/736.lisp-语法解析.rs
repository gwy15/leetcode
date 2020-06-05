/*
 * @lc app=leetcode.cn id=736 lang=rust
 *
 * [736] Lisp 语法解析
 */
struct Solution;
// @lc code=start
type TokenExpression = Vec<Token>;
#[derive(Clone, Debug)]
enum Token {
    Symbol(String),
    Number(i32),
    Expression(TokenExpression),
}

#[allow(unused)]
impl Token {
    pub fn from(expression: String) -> TokenExpression {
        let mut stack = Vec::new();

        let mut cur_expr = Vec::new();
        let mut word = String::new();

        macro_rules! push_word {
            () => {
                if word.len() > 0 {
                    let token = match word.parse::<i32>() {
                        Ok(n) => Token::Number(n),
                        Err(_) => Token::Symbol(word),
                    };
                    cur_expr.push(token);
                    word = "".into();
                }
            };
        }

        for ch in expression.chars() {
            match ch {
                '(' => {
                    push_word!();
                    // push
                    stack.push(cur_expr);
                    cur_expr = Vec::new();
                }
                ')' => {
                    push_word!();
                    // pop stack
                    let mut parent_expr = stack.pop().unwrap();
                    parent_expr.push(Token::Expression(cur_expr));
                    cur_expr = parent_expr;
                }
                ' ' => {
                    push_word!();
                }
                _ => {
                    word.push(ch);
                }
            }
        }
        push_word!();

        debug_assert_eq!(stack.len(), 0);
        cur_expr
    }
}

#[derive(Clone, Debug)]
enum Expression {
    Integer(i32),
    Add(Box<Expression>, Box<Expression>),
    Multi(Box<Expression>, Box<Expression>),
    Var(String),
    Let(Vec<(String, Expression)>, Box<Expression>),
}

#[allow(unused)]
impl Expression {
    pub fn from(token: &Token) -> Self {
        match token {
            Token::Expression(expr) => Self::from_expr(expr),
            Token::Number(n) => Expression::Integer(*n),
            Token::Symbol(sym) => Expression::Var(sym.clone()),
        }
    }

    pub fn from_expr(expr: &TokenExpression) -> Self {
        if expr.len() == 1 {
            return Self::from(&expr[0]);
        }
        match &expr[0] {
            Token::Expression(expr) => Self::from_expr(expr),
            Token::Number(n) => Expression::Integer(*n),
            Token::Symbol(sym) => match &sym[..] {
                "add" => Expression::Add(
                    Box::new(Expression::from(&expr[1])),
                    Box::new(Expression::from(&expr[2])),
                ),
                "mult" => Expression::Multi(
                    Box::new(Expression::from(&expr[1])),
                    Box::new(Expression::from(&expr[2])),
                ),
                "let" => {
                    let length = expr.len();
                    debug_assert!(length % 2 == 0);
                    // first one for let, last one for expr
                    let mut definations = Vec::new();
                    for i in 0..(length / 2 - 1) {
                        if let Token::Symbol(var) = &expr[2 * i + 1] {
                            let pair = (var.clone(), Expression::from(&expr[2 * i + 2]));
                            definations.push(pair);
                        } else {
                            panic!("Illegal var name in let statement");
                        }
                    }

                    Expression::Let(definations, Box::new(Expression::from(&expr[length - 1])))
                }
                var => Expression::Var(sym.clone()),
            },
        }
    }

    pub fn eval(&self, ctx: &mut Ctx) -> i32 {
        match self {
            Expression::Integer(n) => *n,
            Expression::Add(lhs, rhs) => lhs.eval(ctx) + rhs.eval(ctx),
            Expression::Multi(lhs, rhs) => lhs.eval(ctx) * rhs.eval(ctx),
            Expression::Var(var) => *ctx.map[var].last().unwrap(),
            Expression::Let(defs, expr) => {
                // set context
                let mut scoped_vars = Vec::new();
                for (var_name, var_expr) in defs {
                    let var_value = var_expr.eval(ctx);
                    scoped_vars.push(var_name);
                    ctx.map
                        .entry(var_name.clone())
                        .or_insert(Vec::new())
                        .push(var_value);
                }

                let ans = expr.eval(ctx);
                // clear context
                for var_name in scoped_vars.into_iter() {
                    let vals = ctx.map.get_mut(var_name).unwrap();
                    if vals.len() == 1 {
                        ctx.map.remove(var_name);
                    } else {
                        vals.pop();
                    }
                }

                ans
            }
        }
    }
}

use std::collections::HashMap;
struct Ctx {
    pub map: HashMap<String, Vec<i32>>,
}
#[allow(unused)]
impl Ctx {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[allow(unused)]
impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        eprintln!("{}", expression);
        let token_expr = Token::from(expression);
        eprintln!("{:?}", token_expr);
        let expr = Expression::from_expr(&token_expr);
        eprintln!("{:?}", expr);
        let mut ctx = Ctx::new();
        eprintln!();
        expr.eval(&mut ctx)
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($expr:expr, $ans:expr) => {
            assert_eq!(Solution::evaluate($expr.into()), $ans);
        };
    };
    test!("(add 1 2)", 3);
    test!("(mult 3 (add 2 3))", 15);
    test!("(let x 2 (mult x 5))", 10);
    test!("(let x 2 (mult x (let x 3 y 4 (add x y))))", 14);
    test!("(let x 3 x 2 x)", 2);
    test!("(let x 1 y 2 x (add x y) (add x y))", 5);
    test!("(let x 2 (add (let x 3 (let x 4 x)) x))", 6);
    test!("(let a1 3 b2 (add a1 1) b2)", 4);
    test!("(1)", 1);
    test!("1", 1);
}
