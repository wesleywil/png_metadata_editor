
const { invoke, convertFileSrc } = window.__TAURI__.tauri
const { Body } = window.__TAURI__.http;
const { open } = window.__TAURI__.dialog;

document.addEventListener("DOMContentLoaded", function () {
    // Show uploaded image
    const imgContainer = document.getElementById("img_container");
    const imgTitle = document.getElementById("title");
    const imgParams = document.getElementById("text_params");
    const dataParams = document.getElementById("generation_data_params");
    const uploadImage = document.getElementById("btn_upload_img");
    const btnUpdate = document.getElementById("btn_update");
    let openFile = "";

    uploadImage.addEventListener("click", async () => {
        const removeImg = document.getElementById("img_upload_ig");
        if (removeImg) {
            removeImg.remove();
        }

        try {
            openFile = await open({
                multiple: false,
                filters: [{
                    name: "img",
                    extensions: ['png']
                }]
            });
            const res = await invoke("upload_img", { file: openFile });
            const img = document.createElement("img");
            img.id = "img_upload_ig";
            img.src = convertFileSrc(openFile)
            img.className = "img_upload";
            imgContainer.appendChild(img);
            imgTitle.value = res[0].replace(/^.*[\\\/]/, '');
            imgParams.value = res[1];
            dataParams.value = res[2];

        } catch (err) {
            console.log('Error: ----> ', err);
        }
    });

    // Only for test
    btnUpdate.addEventListener("click", () => {
        console.log("Parameters: ", imgParams.value);
        console.log("Data Parameters: ", dataParams.value);
        const test_decode_encode = async () => {
            try {

                const test = await invoke("png_metadata_edit", { parameters: imgParams.value, dataGeneration: dataParams.value, filePath: openFile });
                console.log("Result ===> ", test);
            } catch (err) {
                console.log("Error in Decode/Encode ===> ", err);
            }
        }

        test_decode_encode()
    })




});