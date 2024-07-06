const express = require('express');
const { execFile } = require('child_process');
const path = require('path');
const app = express();
const port = 3000;

app.use(express.json());

app.get('/getStorageProof', (req, res) => {
    const masterSafeAddress = req.query.masterSafeAddress;
    const msgHash = req.query.msgHash;

    if (!masterSafeAddress || !msgHash) {
        return res.status(400).json({ error: 'Missing masterSafeAddress or msgHash parameter' });
    }

    const rustExecutable = path.resolve(__dirname, '../target/release/halo2-multisig');

    execFile(rustExecutable, [masterSafeAddress, msgHash], (error, stdout, stderr) => {
        if (error) {
            console.error(`Error: ${error.message}`);
            return res.status(500).json({ error: 'Internal Server Error' });
        }

        if (stderr) {
            console.error(`stderr: ${stderr}`);
            return res.status(500).json({ error: 'Internal Server Error' });
        }

        let proof;
        try {
            proof = JSON.parse(stdout);
        } catch (e) {
            console.error(`Failed to parse JSON: ${e.message}`);
            return res.status(500).json({ error: 'Internal Server Error' });
        }

        res.json({ masterSafeAddress, msgHash, proof });
    });
});

app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});
