import png from "png-metadata";

const pngFilePath = './test.png';

const readPng = png.readFileSync(pngFilePath);

const list = png.splitChunk(readPng);

const text = list.filter((item)=> item.type === "tEXt" && item.data.startsWith('parameters'));

console.log(text)

