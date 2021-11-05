import React from "react";

export default function PiData(props){
    let cpu = props.cpu;
    let gpu = props.gpu;
    let ram = props.ram;
    
    function getCPUData(){
        return(
            <div>
                <h4>CPU Data: { cpu }</h4>
            </div>
        )
    };
    
    function getGPUData(){
        return(
            <div>
                <h4>GPU Data: { gpu }</h4>
            </div>
        )
    };

    function getRAMData(){
        return(
            <div>
                <h4>RAM Data: { ram }</h4>
            </div>
        )
    };
    
    return(
        <div>
            <div>
                <h2> { getCPUData() } </h2>
            </div>
            <div>
                <h2> { getGPUData() } </h2>
            </div>
            <div>
                <h2> { getRAMData() } </h2>
            </div>
        </div>
    )
};