import {
    createConnection,
    TextDocuments,
    DiagnosticSeverity,
    ProposedFeatures,
    InitializeParams,
    DidChangeConfigurationNotification,
    CompletionItem,
    CompletionItemKind,
    TextDocumentSyncKind,
    InitializeResult,
    DocumentDiagnosticReportKind,
    type DocumentDiagnosticReport,
    Diagnostic,
    TextDocumentPositionParams,
    CompletionItemLabelDetails
} from 'vscode-languageserver/node';
import { TextDocument } from 'vscode-languageserver-textdocument'
import { stringify } from 'querystring';

// IPC transport with proposed features
const connection = createConnection(ProposedFeatures.all)

// Document manager
const documents = new TextDocuments(TextDocument)

let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;
let hasDiagnosticRelatedInformationCapability = false;

connection.onInitialize((params: InitializeParams) => {
    const capabilities = params.capabilities;
    // Check client copabilities
    hasConfigurationCapability = !!(
        capabilities.workspace && !!capabilities.workspace.configuration
    );
    hasWorkspaceFolderCapability = !!(
        capabilities.workspace && !!capabilities.workspace.workspaceFolders
    );
    hasDiagnosticRelatedInformationCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.publishDiagnostics &&
        capabilities.textDocument.publishDiagnostics.relatedInformation
    );
    // Result for client
    const result: InitializeResult = {
        capabilities: {
            textDocumentSync: TextDocumentSyncKind.Incremental,
            completionProvider: {
                resolveProvider: true
            },
            diagnosticProvider: {
                interFileDependencies: false,
                workspaceDiagnostics: false
            }
        }
    };
    if (hasWorkspaceFolderCapability) {
        result.capabilities.workspace = {
            workspaceFolders: {
                supported: true
            }
        };
    }
    return result
});

connection.onInitialized(() => {
    // Watch configuration change
    if (hasConfigurationCapability) {
        connection.client.register(DidChangeConfigurationNotification.type, undefined);
    }
    // Watch workspace folder
    // TODO: Temprory to see it's needed or not
    if (hasWorkspaceFolderCapability) {
        connection.workspace.onDidChangeWorkspaceFolders(_event => {
            connection.console.log('Workspace folder change event received.');
        });
    }
});

// Example settings
// TODO: Complete or remove
interface EnhExSettings {
    maxNumberOfProblems: number;
}

// Fallback for when workspace.configuration is not available
const defaultSettings: EnhExSettings = {
    maxNumberOfProblems: 1000
};
let globalSettings: EnhExSettings = defaultSettings;

// Cache the settings of all open documents
const documentSettings = new Map<string, Thenable<EnhExSettings>>();

connection.onDidChangeConfiguration(change => {
    if (hasConfigurationCapability) {
        // Reset cache
        documentSettings.clear();
    } else {
        globalSettings = (
            (change.settings.enhex || defaultSettings)
        );
    }
    // Refresh diagnostics to apply configurations
    // TODO: Add compare before refresh to optimize this section if needed
    connection.languages.diagnostics.refresh();
});

function getDocumentSettings(resource: string): Thenable<EnhExSettings> {
    if (!hasConfigurationCapability) {
        return Promise.resolve(globalSettings);
    }
    let result = documentSettings.get(resource);
    if (!result) {
        result = connection.workspace.getConfiguration({
            scopeUri: resource,
            section: 'enhex'
        });
        documentSettings.set(resource, result);
    }
    return result;
}

// Remove closed documents cache
documents.onDidClose(e => {
    documentSettings.delete(e.document.uri);
});

connection.languages.diagnostics.on(async (params) => {
    const document = documents.get(params.textDocument.uri);
    if (document !== undefined) {
        return {
            kind: DocumentDiagnosticReportKind.Full,
            // TODO: Add check here for possible same problems and DocumentDiagnosticReportKind.Unchanged
            items: await validateDocument(document)
        } satisfies DocumentDiagnosticReport;
    } else {
        // Don't report problems for unknown documents
        return {
            kind: DocumentDiagnosticReportKind.Full,
            items: [],
        } satisfies DocumentDiagnosticReport;
    }
});

// On first open or content change
documents.onDidChangeContent(change => {
    validateDocument(change.document);
});

async function validateDocument(textDocument: TextDocument): Promise<Diagnostic[]> {
    // TODO: Simplify settings getting and do it less than now
    const settings = await getDocumentSettings(textDocument.uri);

    const text = textDocument.getText();

    // Example check
    const pattern = /\b[A-Z]{2,}\b/g;
    let m: RegExpExecArray | null;
    let problems = 0;
    const diagnostics: Diagnostic[] = [];
    while ((m = pattern.exec(text)) && problems < settings.maxNumberOfProblems) {
        problems++;
        const diagnostic: Diagnostic = {
            severity: DiagnosticSeverity.Warning,
            range: {
                start: textDocument.positionAt(m.index),
                end: textDocument.positionAt(m.index + m[0].length)
            },
            message: `${m[0]} is all uppercase.`,
            source: 'enhex'
        };
        if (hasDiagnosticRelatedInformationCapability) {
            diagnostic.relatedInformation = [
                {
                    location: {
                        uri: textDocument.uri,
                        range: Object.assign({}, diagnostic.range)
                    },
                    message: 'Example related information'
                }
            ];
        }
        diagnostics.push(diagnostic);
    }
    return diagnostics;
}

// TODO: Remove (with watcher) or use
connection.onDidChangeWatchedFiles(_change => {
    connection.console.log('File change event received.')
});

interface SymbolSpec {
    kind: CompletionItemKind
    detail: string
    docs: string
}

const symbolSpecs: Record<string, SymbolSpec> = {
    digit: {
        kind: CompletionItemKind.Constant,
        detail: 'Number Digit: \\d',
        docs: 'Matches a single number character from 0 to 9'
    },
    non_digit: {
        kind: CompletionItemKind.Constant,
        detail: 'Non-Digit: \\D',
        docs: 'Matches any character except 0-9'
    },
    word_char: {
        kind: CompletionItemKind.Constant,
        detail: 'Word character: \\w',
        docs: 'Matches a letter, digit, or underscore'
    },
    non_word_char: {
        kind: CompletionItemKind.Constant,
        detail: 'Non-Word: \\W',
        docs: 'Matches any character except letter, digit, and underscore'
    },
    whitespace: {
        kind: CompletionItemKind.Constant,
        detail: 'Whitespace: \\s',
        docs: 'Matches a space, tab, or newline'
    },
    non_whitespace: {
        kind: CompletionItemKind.Constant,
        detail: 'Non-Whitespace: \\S',
        docs: 'Matches any character except space, tab, and newline'
    },
    lowercase: {
        kind: CompletionItemKind.Constant,
        detail: 'Lowercase letter: [a-z]',
        docs: 'Matches a lowercase letter from a to z'
    },
    uppercase: {
        kind: CompletionItemKind.Constant,
        detail: 'Uppercase letter: [A-Z]',
        docs: 'Matches a uppercase letter from A to Z'
    },
    letter: {
        kind: CompletionItemKind.Constant,
        detail: 'Any letter: [a-zA-Z]',
        docs: 'Matches a lowercase or upper case letter'
    },
    anything: {
        kind: CompletionItemKind.Constant,
        detail: 'A character: .',
        docs: 'Matches any single character'
    },
    dot: {
        kind: CompletionItemKind.Constant,
        detail: 'Literal dot: \\.',
        docs: 'RegEx-scaped dot'
    },
    dash: {
        kind: CompletionItemKind.Constant,
        detail: 'Literal dash: \\-',
        docs: `RegEx-scaped dash

Note: To optimize your pattern, literal dash will be scaped just when needed.`
    },
    tab: {
        kind: CompletionItemKind.Constant,
        detail: 'Literal tab: \\t',
        docs: 'Matches a (horizontal) tab character'
    },
    newline: {
        kind: CompletionItemKind.Constant,
        detail: 'Literal newline: \\n',
        docs: 'Matches a newline character (LF)'
    },
    carriage_return: {
        kind: CompletionItemKind.Constant,
        detail: 'Literal carriage-return: \\r',
        docs: 'Matches a carriage return (CR)'
    },
    hex_digit: {
        kind: CompletionItemKind.Constant,
        detail: 'A hex digit from 0 to f: [\da-fA-F]',
        docs: 'Matches a hex digit, numeric, lower, or upper'
    },
    null: {
        kind: CompletionItemKind.Constant,
        detail: 'Null spacial character: \\0',
        docs: 'Matches a ASCII 0 null character'
    },
    vertical_tab: {
        kind: CompletionItemKind.Constant,
        detail: 'Literal vertical tab: \\v',
        docs: 'Matches a vertical tab character'
    },
    form_feed: {
        kind: CompletionItemKind.Constant,
        detail: 'Form feed character: \\f',
        docs: 'Matches a single form feed character'
    },
    bell: {
        kind: CompletionItemKind.Constant,
        detail: 'Bell special character: \\a',
        docs: 'Matches a ASCII 7 bell character'
    },
    backslash: {
        kind: CompletionItemKind.Constant,
        detail: 'Literal backslash: \\\\',
        docs: 'RegEx-scaped backslash character'
    },
    one_or_more: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "One or more ..." : (...)+',
        docs: 'Matches on one or more of inner pattern'
    },
    zero_or_more: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "Zero or more ..." : (...)*',
        docs: 'Matches on zero or more of inner pattern'
    },
    optional: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "Optional ..." : (...)?',
        docs: 'Matches on zero or one of inner pattern'
    },
    exactly: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "Exactly N times ..." : (...){n}',
        docs: 'Matches exactly N times of inner pattern'
    },
    at_least: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "At least N times ..." : (...){n,}',
        docs: 'Matches at least N times of inner pattern'
    },
    between: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "Between N and M times ..." : (...){n,m}',
        docs: 'Matches between N and M times (inclusive) of inner pattern'
    },
    one_or_more_lazy: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "One or more (lazy) ..." : (...)+?',
        docs: 'Matches on one or more of inner pattern (non-greedy)'
    },
    zero_or_more_lazy: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "Zero or more (lazy) ..." : (...)*?',
        docs: 'Matches on zero or more of inner pattern (non-greedy)'
    },
    optional_lazy: {
        kind: CompletionItemKind.Function,
        detail: 'Quantifier: "Optional (lazy) ..." : (...)??',
        docs: 'Matches on zero or one of inner pattern (non-greedy)'
    },
    group: {
        kind: CompletionItemKind.Function,
        detail: 'Capturing group: (...)',
        docs: 'Creates a capturing group of inner pattern'
    },
    non_capturing: {
        kind: CompletionItemKind.Function,
        detail: 'Non-capturing group: (?:...)',
        docs: 'Creates a non-capturing group of inner pattern'
    },
    named: {
        kind: CompletionItemKind.Function,
        detail: 'Named group: (?P<name>...)',
        docs: 'Creates a named capturing group of inner pattern with the specified name'
    },
    not: {
        kind: CompletionItemKind.Function,
        detail: 'Negated character class: [^...]',
        docs: 'Matches on everything except inner pattern'
    },
    start: {
        kind: CompletionItemKind.Constant,
        detail: 'Start of string/line: ^',
        docs: 'Matches at the beginning of the string or line'
    },
    end: {
        kind: CompletionItemKind.Constant,
        detail: 'End of string/line: $',
        docs: 'Matches at the end of the string or line'
    },
    word_boundary: {
        kind: CompletionItemKind.Constant,
        detail: 'Word boundary: \\b',
        docs: 'Matches at a word boundary (between word and non-word character)'
    },
    followed_by: {
        kind: CompletionItemKind.Function,
        detail: 'Positive lookahead: (?=...)',
        docs: 'Matches if inner pattern matches ahead (without consuming)'
    },
    not_followed_by: {
        kind: CompletionItemKind.Function,
        detail: 'Negative lookahead: (?!...)',
        docs: 'Matches if inner pattern does NOT match ahead (without consuming)'
    },
    preceded_by: {
        kind: CompletionItemKind.Function,
        detail: 'Positive lookbehind: (?<=...)',
        docs: 'Matches if inner pattern matches behind (without consuming)'
    },
    not_preceded_by: {
        kind: CompletionItemKind.Function,
        detail: 'Negative lookbehind: (?<!...)',
        docs: 'Matches if inner pattern does NOT match behind (without consuming)'
    },
    backref: {
        kind: CompletionItemKind.Function,
        detail: 'Backreference (by index / name): \\i / (?P=name)',
        docs: 'Matches the same text as previously matched by a capturing group (string for named, netural number for regular)'
    },
    tld: {
        kind: CompletionItemKind.Function,
        detail: 'Top Level Domain preset',
        docs: 'Matches common TLDs like com, org, net, etc. (2-10 lowercase letters)'
    },
    email: {
        kind: CompletionItemKind.Function,
        detail: 'Email address preset',
        docs: 'Matches RFC 5322 simplified email addresses (local@domain.tld)'
    },
    url: {
        kind: CompletionItemKind.Function,
        detail: 'URL preset',
        docs: 'Matches full URLs including protocol, domain, path, and query parameters'
    },
    ipv4: {
        kind: CompletionItemKind.Function,
        detail: 'IPv4 address preset',
        docs: 'Matches IPv4 addresses (0.0.0.0 to 255.255.255.255)'
    },
};

const symbols: CompletionItem[] = Object.entries(symbolSpecs).map(([label, spec]) => ({
    label: label,
    kind: spec.kind,
    data: label,
    detail: spec.detail,
    documentation: spec.docs
}));

const completionSymbols: CompletionItem[] = symbols.map(
    ({ label, kind, data }) => ({ label, kind, data })
);
const completionMap = new Map<string, CompletionItem>();
for (const s of symbols) {
    completionMap.set(s.label, s);
}

connection.onCompletion(
    (_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
        return completionSymbols;
    }
);

connection.onCompletionResolve(
    (item: CompletionItem): CompletionItem => {
        return completionMap.get(item.label) || item;
    }
);

documents.listen(connection);
connection.listen();
