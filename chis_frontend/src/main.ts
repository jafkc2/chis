const url = "127.0.0.1";

let messages;
let author_name = generate_name();


async function main() {
    document.getElementById("message_input")?.addEventListener("keydown", message_input);
}


async function message_input(event: any){
    if (event.key == "Enter"){
        const message_input = document.getElementById("message_input") as HTMLInputElement;
        const m_text = message_input.value;
        message_input.value = "";
        
        const data = {
            author: author_name,
            text: m_text
        }
     
        const response = await fetch(`http://${url}:3000/api/send_message`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });

        const result = await response.json();
    }
}

function message(author: string, message: string){
    return `<div class="message">
                <div class="icon">${author}</div>
                <div class="bubble">${message}</div>
            </div>
            `
}

setInterval(async () => {
    const messages = await fetch(`http://${url}:3000/api/get_messages`, {
        method: "GET",
        headers: {
            'Content-Type': "application/json",
        },
    });

    const json = await messages.json();
    let html_messages = ""

    console.log(json)
    for (const i of json){
        html_messages += message(i.author, i.text)
    }

    const document_messages = document.getElementById("messages") as HTMLDivElement;
    document_messages.innerHTML = html_messages;

}, 1000);

main();

function generate_name(){
    const consonants = ["b", "c", 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
    const vowels = ['a', 'e', 'i', 'o', 'u'];

    let name = ""

    const name_letter_number = Math.floor(Math.random() * 6 + 2);

    for (let i = 0; i < name_letter_number; i++) {
        const consonant_choice = consonants[Math.floor(Math.random() * consonants.length)];
        const vowel_choice = vowels[Math.floor(Math.random() * vowels.length)];
        name += consonant_choice + vowel_choice
    }

    return name;
}