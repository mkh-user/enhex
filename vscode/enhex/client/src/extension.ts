import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import { DocumentSelector, LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    // Node server
    const serverModule = context.asAbsolutePath(
        path.join('server', 'out', 'server.js')
    );
    // Uses 'debug' for server when client is in debug mode
    const serverOptions: ServerOptions = {
        run: { module: serverModule, transport: TransportKind.ipc },
        debug: { module: serverModule, transport: TransportKind.ipc }
    };
    // Client configuration
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { language: 'enhex' }
            // TODO: Check language injection required configuration
        ],
        synchronize: {
            // Watch .clientrc files
            fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
        }
    };
    // Create client
    client = new LanguageClient(
        'enhex-lsp-server',
        'EnhEx LSP Server',
        serverOptions,
        clientOptions
    );
    // Run client and server
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) return undefined;
    return client.stop();
}
