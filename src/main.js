const { invoke } = window.__TAURI__.tauri;

let date = new Date();
let dater = date.getDate()
let year = date.getFullYear();
let month = date.getMonth();

let offsetsel = 0;

const months = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december"
];

const weekdays = [
    "x",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday"
];

function plusoneday() {
    offsetsel += 1;

    selecteddate()
}

function minusoneday() {
    offsetsel -= 1;

    selecteddate()
}

function selecteddate() {
    let now = new Date();
    let datestring = `${now.getDate()} ${months[now.getMonth()]} ${now.getFullYear()}`
    let selecteddate = new Date(year, month, dater + offsetsel);
    console.log(selecteddate)
    // let currentdate = `${now.getDate()}`
    //now is equal to selected date by user, not the current date
    document.getElementById("selecteddate").innerHTML = datestring;
    document.getElementById("currentdate").innerHTML = `${selecteddate.getDate()} ${months[selecteddate.getMonth()]} ${selecteddate.getFullYear()}`;
}

const manipulate = () => {
 
    // Get the first day of the month
    let dayone = new Date(year, month, 1).getDay();
 
    // Get the last date of the month
    let lastdate = new Date(year, month + 1, 0).getDate();
 
    // Get the day of the last date of the month
    let dayend = new Date(year, month, lastdate).getDay();
 
    // Get the last date of the previous month
    let monthlastdate = new Date(year, month, 0).getDate();

    console.log(`helo !${weekdays[dayone]} ${lastdate} ${weekdays[dayend]} ${monthlastdate}`)
}

window.onload = selecteddate();