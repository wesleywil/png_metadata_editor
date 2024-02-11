
const { invoke } = window.__TAURI__.tauri
const { Body } = window.__TAURI__.http;
const { open } = window.__TAURI__.dialog;

document.addEventListener("DOMContentLoaded", function () {
    // Show uploaded image
    const upload = document.getElementById("png_file");
    const container = document.getElementById("to_remove");

    // upload.addEventListener("change", () => {
    //     const selectedFile = upload.files[0];
    //     const reader = new FileReader();
    //     reader.onload = function (event) {
    //         const imageURL = event.target.result;
    //         const img = document.createElement("img");
    //         img.src = imageURL;
    //         img.className = "upload_img"
    //         container.appendChild(img);
    //     }
    //     reader.readAsDataURL(selectedFile)
    //     console.log(selectedFile);

    //     // Trying to upload image
    //     // const form = new FormData();
    //     // form.append('key', 'value');
    //     // form.append('image', selectedFile);
    //     // const formBody = Body.form(form);


    //     // const test_upload_rust = async () => {
    //     //     try {
    //     //         const openFile = await open({
    //     //             multiple: false,
    //     //             filters: [{
    //     //                 name: "test_test_img",
    //     //                 extensions: ['png']
    //     //             }]
    //     //         });
    //     //         await invoke("upload_img_test", openFile)
    //     //     } catch (error) {
    //     //         console.log("ERROR ", error);
    //     //     }
    //     // }
    //     // test_upload_rust();
    // });
    const imgParams = document.getElementById("text_params");
    const dataParams = document.getElementById("generation_data_params");
    // Rust tauri tests

    // invoke('read_img_test')
    //     .then((response) => {
    //         console.log('RESPONSE FROM INVOKE READ IMG TEST ---> ', response)
    //         imgParams.value = response[0];
    //         dataParams.value = response[1]
    //     })
    //     .catch((error) => {
    //         console.log('ERROR ---> ', error);
    //     })

    const uploadImage = document.getElementById("btn_upload_img");

    uploadImage.addEventListener("click", async () => {
        try {
            const openFile = await open({
                multiple: false,
                filters: [{
                    name: "test_test_img",
                    extensions: ['png']
                }]
            });
            await invoke("upload_img_test", { file: openFile });
        } catch (err) {
            console.log('Error: ----> ', err);
        }
    });
});