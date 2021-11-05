import React from "react";

import PiContainer from "../compontents/PiContainer"

export default function Home() {
    return(
        <div>
            <div>
                <h1 className="bg-gray-200">Pi Monitor</h1>
            </div>
            <div>
                <PiContainer />
            </div>
        </div>
    )

}