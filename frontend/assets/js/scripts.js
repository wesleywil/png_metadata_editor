
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

    uploadImage.addEventListener("click", async () => {
        const removeImg = document.getElementById("img_upload_ig");
        if (removeImg) {
            removeImg.remove();
        }

        try {
            const openFile = await open({
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
});