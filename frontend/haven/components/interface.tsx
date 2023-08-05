"use client"
import React from 'react';
import { Switch } from '@nextui-org/switch'

export default function UI() {
    const [checked, setChecked] = React.useState(false)

    const handleChange = () => {
        setChecked(!checked)
    }
    const t = "test"

    return (
        <div>
            <Switch
                onChange={handleChange}
                value={ checked ? "locked" : "unlocked" }
            />
            <h4> { checked ? "Locked" : "Unlocked" } </h4>
        </div>
    )
}