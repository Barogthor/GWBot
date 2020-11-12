const fs = require('fs');
const fs_promises = require('fs').promises;
const parse = require('csv-parse');
const parse_sync = require('csv-parse/lib/sync');
const http = require('https');

let counter = 0;
const cacheIcon = record => {
    // console.log(record[2])
    // if(counter++>2) return;
    http.get(record[2], (res) => {
        res.setEncoding('binary');
        let rawData = '';
        res.on('data', (chunk) => {
            rawData += chunk;
        });
        res.on('end', () => {
            // console.log("\ndatas:",rawData)
            fs.writeFile(`${__dirname}'/../cache/${record[0]}.jpg`, rawData, {encoding: 'binary'}, (err) => console.error(err))
        })
    }).on('error', (e) => {
        console.error(`Got error: ${e.message}`);
    });
}

(async function () {
    const skills_buffer = await fs_promises.readFile(__dirname + '/../datas/skills.csv')
    const skills = parse_sync(skills_buffer, {delimiter: ';', from_line: 2})
    skills.forEach(cacheIcon)
})()