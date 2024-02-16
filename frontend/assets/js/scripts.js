
const { invoke, convertFileSrc } = window.__TAURI__.tauri
const { Body } = window.__TAURI__.http;
const { open } = window.__TAURI__.dialog;

document.addEventListener("DOMContentLoaded", function () {
    // Show uploaded image
    const imgContainer = document.getElementById("img_container");
    const imgParams = document.getElementById("text_params");
    const containerExtraData = document.getElementById("extra_data");
    const dataParams = document.getElementById("generation_data_params");
    const uploadImage = document.getElementById("btn_upload_img");
    const btnUpdate = document.getElementById("btn_update");
    let btnRemoveExtra = document.getElementById("remove_extra_data");
    let extraData = false;

    btnUpdate.classList.add("hidden")
    let openFile = "";

    // Function allowing users to upload a image and display the metadata
    uploadImage.addEventListener("click", async () => {
        const removeImg = document.getElementById("img_upload_ig");
        if (removeImg) {
            removeImg.remove();
            imgContainer.classList.remove("landscape");
            imgContainer.classList.remove("square")
            btnRemoveExtra.checked = false
            containerExtraData.hidden = false
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
            console.log('Test extraData ==> ', res[2]);
            imgContainer.appendChild(img);
            imgParams.value = res[1];
            if (res[2] === "no parameters extra data") {
                containerExtraData.hidden = true
                btnRemoveExtra.checked = true

            }
            dataParams.value = res[2];

        } catch (err) {
            console.log('Error: ----> ', err);
        }
    });

    // Function allowing users to edit the metadata of a image
    btnUpdate.addEventListener("click", () => {
        const test_decode_encode = async () => {
            try {
                const test = await invoke("png_metadata_edit", { parameters: imgParams.value, dataGeneration: dataParams.value, noExtraData: extraData, filePath: openFile });
                if (test === null) {
                    window.location.href = "done.html"
                }
            } catch (err) {
                console.log("Error in Decode/Encode ===> ", err);
            }
        }

        test_decode_encode()
    })

    btnRemoveExtra.addEventListener("change", function () {
        if (this.checked) {
            extraData = this.checked;
            containerExtraData.hidden = true

        } else {
            extraData = this.checked;
            containerExtraData.hidden = false
        }
    })
});