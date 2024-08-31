'use client'

import { useEffect, useState } from 'react';
// import { invoke } from '@tauri-apps/api/core';
import { invoke } from '@tauri-apps/api/tauri'

export default function Greet() {
    const [fired, setFired] = useState(false);
  const [greeting, setGreeting] = useState('0');


  useEffect(() => {

      if (!fired && greeting === '0') {
          invoke('date').then(r => {
              console.log("fired")
              setGreeting(r)
              invoke('write_string', {string: greeting}).then(r => {
                  console.log(r)
              })
              setFired(true)
          })
          if (fired) {
              invoke('printall').then(r => {
                  console.log(r)
              })
          }

      }
  }, [fired, setFired, greeting, setGreeting])


  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}