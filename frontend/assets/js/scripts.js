
const { invoke, convertFileSrc } = window.__TAURI__.tauri
const { Body } = window.__TAURI__.http;
const { open } = window.__TAURI__.dialog;

document.addEventListener("DOMContentLoaded", function () {
    // Show uploaded image
    const imgContainer = document.getElementById("img_container");
    const imgParams = document.getElementById("text_params");
    const dataParams = document.getElementById("generation_data_params");
    const uploadImage = document.getElementById("btn_upload_img");
    const btnUpdate = document.getElementById("btn_update");
    btnUpdate.classList.add("hidden")
    let openFile = "";

    uploadImage.addEventListener("click", async () => {
        const removeImg = document.getElementById("img_upload_ig");
        if (removeImg) {
            removeImg.remove();
            imgContainer.classList.remove("landscape");
            imgContainer.classList.remove("square")
        }

        try {
            openFile = await open({
                multiple: false,
                filters: [{
                    name: "img",
                    extensions: ['png']
                }]
            });
            btnUpdate.classList.remove("hidden")
            const res = await invoke("upload_img", { file: openFile });
            const img = document.createElement("img");
            img.id = "img_upload_ig";
            img.src = convertFileSrc(openFile)
            img.className = "img_upload";
            img.onload = function () {
                if (img.naturalWidth > img.naturalHeight) {
                    imgContainer.classList.add("landscape");
                } else if (img.naturalWidth === img.naturalHeight) {
                    imgContainer.classList.add("square")
                }
            }
            imgContainer.appendChild(img);
            imgParams.value = res[1];
            dataParams.value = res[2];

        } catch (err) {
            console.log('Error: ----> ', err);
        }
    });

    // Edit Png metadata
    btnUpdate.addEventListener("click", () => {
        const test_decode_encode = async () => {
            try {
                const test = await invoke("png_metadata_edit", { parameters: imgParams.value, dataGeneration: dataParams.value, filePath: openFile });
                if (test === null) {
                    window.location.href = "done.html"
                }
                console.log("Result ===> ", test);
            } catch (err) {
                console.log("Error in Decode/Encode ===> ", err);
            }
        }

        test_decode_encode()
    })
});