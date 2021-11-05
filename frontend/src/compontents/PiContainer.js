import React from "react";

import PiData from "./PiData.js"

export default function PiContainer(){
 
    async function GetPiInformation(){
        const url = process.env.GETPIINFORMATION;
        const http = new XMLHttpRequest();
        http.open("GET", url); 
        http.send()
        http.onreadystatechange = (err) => {
            console.log(http.responseText)
        }
        //response returns to PiItems component
    };

    function sampledata(){
        let sample = {
            "pis":[
                {
                    "id":0,
                    "cpu":90,
                    "gpu":10,
                    "ram":200
                },
                {
                    "id":1,
                    "cpu":10,
                    "gpu":40,
                    "ram":10
                }
            ]
        }
        return sample;

    }

    function PiItems(){
        // Get a list of pi ids.
        //let list_pi = GetPiInformation() 
        let sample = sampledata()
        // pass pi information to PiData for each pi
        // Try using context for future use
        return sample.pis.map((pi) => (
            <div>  
                <h3> Pi Number: { pi.id } </h3> 
                <PiData cpu={ pi.cpu } gpu={ pi.gpu } ram={ pi.ram }/>
            </div>
        ));
    }
    return(
        <div className="bg-blue-200">
            <h3>Hello from PiContainer</h3>
            <div>
                { PiItems() }
            </div>
        </div>
    )
};