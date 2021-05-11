const fs = require('fs')
const lipsum = require('./lipsum.js')

let orgfile = '* Wrapper\n'
const fnarr = (()=>{
    // https://stackoverflow.com/questions/2450954/how-to-randomize-shuffle-a-javascript-array
    function shuffle(array) {
        var currentIndex = array.length, temporaryValue, randomIndex;

        // While there remain elements to shuffle...
        while (0 !== currentIndex) {

            // Pick a remaining element...
            randomIndex = Math.floor(Math.random() * currentIndex);
            currentIndex -= 1;

            // And swap it with the current element.
            temporaryValue = array[currentIndex];
            array[currentIndex] = array[randomIndex];
            array[randomIndex] = temporaryValue;
        }

        return array;
    }

    let arr = []
    for(let i=1; i<lipsum.length+1; i++){
        arr.push(i)
    }
    return shuffle(arr)
})()

for(let i=0; i<lipsum.length; i++){
    orgfile += `${lipsum[i]}[fn:${fnarr[i]}]\n`
}

orgfile += '\n* Footnotes\n'

for(let i=0; i<lipsum.length; i++){
    orgfile += `[fn:${fnarr[i]}]\nSome citation text, possibly with some names, who knows, I just need the space with some randomisation ${Math.random()} in case emacs checks for similar lines\n\n`
}

fs.writeFile('./broken-footnotes.org',orgfile,console.log)
