document.addEventListener("DOMContentLoaded", function() {
    // Show uploaded image
    const upload = document.getElementById("png_file");
    const container = document.getElementById("to_remove");
    const imgParams = document.getElementById("text_params");
    upload.addEventListener("change", () => {
        const selectedFile = upload.files[0];
        const reader = new FileReader();
        reader.onload = function(event){
            const imageURL = event.target.result;
            const img = document.createElement("img");
            img.src = imageURL;
            img.className = "upload_img"
            container.appendChild(img);
        }
        reader.readAsDataURL(selectedFile)
        console.log(selectedFile);

    });

    // Rust tauri tests
    const {invoke} = window.__TAURI__.tauri
    invoke('greet', {name:'HOLYSHIT'})
        .then((response)=>{
            console.log('RESPONSE FROM INVOKE GREET ---> ', response)
        })
});