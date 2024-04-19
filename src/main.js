const { invoke } = window.__TAURI__.tauri;

let date = new Date();
let year = date.getFullYear();
let month = date.getMonth();

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

function selecteddate() {
    let now = new Date();
    let datestring = `${now.getDate()} ${months[now.getMonth()]} ${now.getFullYear()}`
    document.getElementById("selecteddate").innerHTML = datestring;
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

    console.log("helo !${weekdays[dayone]} ${lastdate} ${weekdays[dayend]} ${monthlastdate}")
}

// window.onload = selecteddate;