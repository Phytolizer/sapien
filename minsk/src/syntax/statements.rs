use super::expressions::ExpressionSyntax;
use super::ElseClauseSyntax;
use super::SyntaxKind;
use super::SyntaxNodeRef;
use super::SyntaxToken;

#[derive(Debug, Clone)]
pub enum StatementSyntax {
    Block(BlockStatementSyntax),
    Expression(ExpressionStatementSyntax),
    For(ForStatementSyntax),
    If(IfStatementSyntax),
    VariableDeclaration(VariableDeclarationStatementSyntax),
    While(WhileStatementSyntax),
}

impl StatementSyntax {
    pub fn create_ref(&self) -> StatementSyntaxRef {
        match self {
            StatementSyntax::Block(s) => StatementSyntaxRef::Block(s),
            StatementSyntax::Expression(s) => StatementSyntaxRef::Expression(s),
            StatementSyntax::For(s) => StatementSyntaxRef::For(s),
            StatementSyntax::If(s) => StatementSyntaxRef::If(s),
            StatementSyntax::VariableDeclaration(s) => StatementSyntaxRef::VariableDeclaration(s),
            StatementSyntax::While(s) => StatementSyntaxRef::While(s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StatementSyntaxRef<'a> {
    Block(&'a BlockStatementSyntax),
    Expression(&'a ExpressionStatementSyntax),
    For(&'a ForStatementSyntax),
    If(&'a IfStatementSyntax),
    VariableDeclaration(&'a VariableDeclarationStatementSyntax),
    While(&'a WhileStatementSyntax),
}

impl<'a> StatementSyntaxRef<'a> {
    pub(crate) fn kind(&self) -> SyntaxKind {
        match self {
            StatementSyntaxRef::Block(_) => SyntaxKind::BlockStatement,
            StatementSyntaxRef::Expression(_) => SyntaxKind::ExpressionStatement,
            StatementSyntaxRef::For(_) => SyntaxKind::ForStatement,
            StatementSyntaxRef::If(_) => SyntaxKind::IfStatement,
            StatementSyntaxRef::VariableDeclaration(_) => SyntaxKind::VariableDeclarationStatement,
            StatementSyntaxRef::While(_) => SyntaxKind::WhileStatement,
        }
    }

    pub(super) fn children(self) -> Vec<SyntaxNodeRef<'a>> {
        match self {
            StatementSyntaxRef::Block(s) => {
                let mut result = vec![SyntaxNodeRef::Token(&s.open_brace_token)];
                result.append(
                    &mut s
                        .statements
                        .iter()
                        .map(|s| SyntaxNodeRef::Statement(s.create_ref()))
                        .collect(),
                );
                result.push(SyntaxNodeRef::Token(&s.close_brace_token));
                result
            }
            StatementSyntaxRef::Expression(s) => {
                vec![SyntaxNodeRef::Expression(s.expression.create_ref())]
            }
            StatementSyntaxRef::For(s) => vec![
                SyntaxNodeRef::Token(&s.for_keyword),
                SyntaxNodeRef::Token(&s.identifier_token),
                SyntaxNodeRef::Token(&s.equals_token),
                SyntaxNodeRef::Expression(s.lower_bound.create_ref()),
                SyntaxNodeRef::Token(&s.to_keyword),
                SyntaxNodeRef::Expression(s.upper_bound.create_ref()),
                SyntaxNodeRef::Statement(s.body.create_ref()),
            ],
            StatementSyntaxRef::If(s) => {
                let mut result = vec![
                    SyntaxNodeRef::Token(&s.if_keyword),
                    SyntaxNodeRef::Expression(s.condition.create_ref()),
                    SyntaxNodeRef::Statement(s.then_statement.create_ref()),
                ];
                if let Some(else_clause) = &s.else_clause {
                    result.push(SyntaxNodeRef::ElseClause(else_clause));
                }
                result
            }
            StatementSyntaxRef::VariableDeclaration(s) => vec![
                SyntaxNodeRef::Token(&s.keyword),
                SyntaxNodeRef::Token(&s.identifier),
                SyntaxNodeRef::Token(&s.equals_token),
                SyntaxNodeRef::Expression(s.initializer.create_ref()),
            ],
            StatementSyntaxRef::While(s) => vec![
                SyntaxNodeRef::Token(&s.keyword),
                SyntaxNodeRef::Expression(s.condition.create_ref()),
                SyntaxNodeRef::Statement(s.body.create_ref()),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatementSyntax {
    pub open_brace_token: SyntaxToken,
    pub statements: Vec<StatementSyntax>,
    pub close_brace_token: SyntaxToken,
}

#[derive(Debug, Clone)]
pub struct ExpressionStatementSyntax {
    pub expression: ExpressionSyntax,
}

#[derive(Debug, Clone)]
pub struct ForStatementSyntax {
    pub for_keyword: SyntaxToken,
    pub identifier_token: SyntaxToken,
    pub equals_token: SyntaxToken,
    pub lower_bound: ExpressionSyntax,
    pub to_keyword: SyntaxToken,
    pub upper_bound: ExpressionSyntax,
    pub body: Box<StatementSyntax>,
}

#[derive(Debug, Clone)]
pub struct IfStatementSyntax {
    pub if_keyword: SyntaxToken,
    pub condition: ExpressionSyntax,
    pub then_statement: Box<StatementSyntax>,
    pub else_clause: Option<ElseClauseSyntax>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclarationStatementSyntax {
    pub keyword: SyntaxToken,
    pub identifier: SyntaxToken,
    pub equals_token: SyntaxToken,
    pub initializer: ExpressionSyntax,
}

#[derive(Debug, Clone)]
pub struct WhileStatementSyntax {
    pub keyword: SyntaxToken,
    pub condition: ExpressionSyntax,
    pub body: Box<StatementSyntax>,
}
