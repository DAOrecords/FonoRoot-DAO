import React, { useState } from 'react';
import Nav from './Nav';


export default function TestPageTwo() {
  const [imageCID, setImageCID] = useState("QmWUrGSGbjdft3X5EL56Ldf11htPUiGrmzMfhYUJfU6jsQ");
  const [musicCID, setMusicCID] = useState("QmWUrGSGbjdft3X5EL56Ldf11htPUiGrmzMfhYUJfU6jsQ");
  const [metaCID, setMetaCID] = useState("QmWUrGSGbjdft3X5EL56Ldf11htPUiGrmzMfhYUJfU6jsQ");

  function saveData() {
    window.alert("This action should create a unique ID, and that ID should be saved somewhere, if we want to update the data. Maybe LocalStorage. Of course, it's also important that the same AccountId is updating the field.")
  }


  return (
    <>
      <Nav />
      <main>
        <h2>Prepairing Data</h2>

        <section>
          <p>{"First the Artist will upload the metadata to the contract."}</p>
          <p>{"It is possible to prepair all the data on front end, and only upload it in 1 action."}</p>
          <p>{"Or, the user could quit uploading information, and come back later."}</p>
          <p>{"It is possible to preview the uploaded info, and to change things."}</p>
          <p>{"After all the info is uploaded, (media is already on IPFS), the Artist can initiate the minting of the NFT (the MotherContract will do the minting)."}</p>
          <p>{"Alternatively, the minting can be scheduled, either Croncat will do the minting or the MotherContrat, all the data is already stored in the DAO contract"}</p>
          <p>{"After the NFT is minted, the temporary data will be deleted."}</p>
          <p>{".  "}</p>
          <p>{"We won't simulate the uploading of the data here, only the saving of the final CID to the MotherContract"}</p>
          

          <label>Image CID</label>
          <input className="cidInput" value={imageCID} onChange={(e) => setImageCID(e.target.value)}></input><br></br>

          <label>Music Folder CID</label>
          <input className="cidInput" value={musicCID} onChange={(e) => setMusicCID(e.target.value)}></input><br></br>

          <label>Meta JSON CID</label>
          <input className="cidInput" value={metaCID} onChange={(e) => setMetaCID(e.target.value)}></input><br></br>

          <button onClick={saveData}>Save Data</button>

        </section>
      </main>
    </>
  )
}
