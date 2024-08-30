'use client'

import { useEffect, useState } from 'react';
// import { invoke } from '@tauri-apps/api/core';
import { invoke } from '@tauri-apps/api/tauri'

export default function Greet() {
    const [fired, setFired] = useState(false);
  const [greeting, setGreeting] = useState('0');

  invoke('date').then(r => {
      console.log("fired")
      setGreeting(r)
  })
    invoke('write_string', {string: greeting.toString()}).then(r => {
        console.log(r)
    })
  // useEffect(() => {
  //   invoke<string>('date', { name: 'Next.js' })
  //     .then(result => {
  //       console.log("fired")
  //       setGreeting(result)
  //     })
  //     .catch(console.error)
  // }, [])


  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}