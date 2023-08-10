"use client"
import React from 'react';
import { Switch } from '@nextui-org/switch'

function toggleStatus (status = {}){
    // const response = fetch('http://0.0.0.0:8080/api/status/door')
    // const data = response.json
    // return data
    return fetch('http://0.0.0.0:8080/api/status/door', {
        method: "POST",
        mode: "cors",
        credentials: "same-origin",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ status: !status })
    })
        .then(response => response.json())
        // .then(data => data.status)
}

export default function UI() {
    const [checked, setChecked] = React.useState(false)

    const handleChange = async () => {
        // setChecked(!checked)
        const response = await toggleStatus(checked)
        const status = response.status
        // alert(`Is this your full name: ${response.status}`)
        setChecked(status)
    }

    return (
        <div>
            <Switch
                isSelected={ checked}
                onChange={handleChange}
                value={ checked ? "locked" : "unlocked" }
            />
            <h4> { checked ? "Locked" : "Unlocked" } </h4>
        </div>
    )
}