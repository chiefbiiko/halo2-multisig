const express = require('express');
const { spawn } = require('child_process');
const path = require('path');
const app = express();
const port = 3000;

app.use(express.json());

const options = {
    env: {
        RUST_LOG: 'info',
        RUST_BACKTRACE: 'full'
    }
};

app.get('/getStorageProof', (req, res) => {
    const masterSafeAddress = req.query.masterSafeAddress;
    const msgHash = req.query.msgHash;

    if (!masterSafeAddress || !msgHash) {
        return res.status(400).json({ error: 'Missing masterSafeAddress or msgHash parameter' });
    }

    const rustExecutable = path.resolve(__dirname, '../target/release/halo2-multisig');

    console.log("Running prover");

    const child = spawn(rustExecutable, [masterSafeAddress, msgHash], options);

    let scriptOutput = "";

    child.stdout.setEncoding('utf8');
    child.stdout.on('data', (data) => {
        console.log('stdout: ' + data);
        scriptOutput += data.toString();
    });

    child.stderr.setEncoding('utf8');
    child.stderr.on('data', (data) => {
        console.error('stderr: ' + data);
        scriptOutput += data.toString();
    });

    child.on('close', (code) => {
        console.log('closing code: ' + code);
        console.log('Full output of script: ', scriptOutput);

        try {
            const proof = extractEvmCalldata(scriptOutput);
            if (!proof) {
                return res.status(500).json({ error: 'Failed to find evm_calldata in the output' });
            }
            res.json({ masterSafeAddress, msgHash, proof });
        } catch (e) {
            console.error(`Failed to process output: ${e.message}`);
            res.status(500).json({ error: 'Internal Server Error' });
        }
    });

    child.on('error', (error) => {
        console.error(`Failed to start subprocess: ${error.message}`);
        res.status(500).json({ error: 'Internal Server Error' });
    });
});

function extractEvmCalldata(output) {
    const lines = output.split('\n');
    for (const line of lines) {
        if (line.includes('evm_calldata')) {
            const match = line.match(/evm_calldata: (.*)/);
            if (match) {
                return match[1].trim();
            }
        }
    }
    return null;
}

app.listen(port, '0.0.0.0', () => {
    console.log(`Server running at http://localhost:${port}`);
});
