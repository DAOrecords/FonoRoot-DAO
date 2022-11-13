import React, { useState } from 'react';
import Nav from '../Nav';


export default function TestPageFour() {



  return (
    <>
      <Nav />
      <main>
        <h2>Minting</h2>

        <section>
          <p>{"The Artist can initiate the minting of the NFT"}</p>
        </section>

        <section>
          <p>{"Or, the Artist can schedule the minting of the NFT (through CronCat or othe means)"}</p>
        </section>
      </main>
    </>
  )
}
