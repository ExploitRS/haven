import React from "react";
import { Switch } from '@nextui-org/switch';

export default function App() {
  return (
    <div>
      <Switch defaultSelected aria-label="Automatic updates" size='lg' className='w-96' />
    </div>
  );
}