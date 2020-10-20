var home = require('fs').readFileSync(__dirname + "/index.html");

module.exports = (req, res) => {
    res.setHeader('content-type', 'text/html');
    res.status(200).send(home);
}