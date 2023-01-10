import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, getLastProposalId, getListOfAllInProgressNfts, getListOfProposals, updateNft } from '../utils';


export default function TestPageThree() {
  const [contract, setContract] = useState("");
  const [title, setTitle] = useState("");
  const [desc, setDesc] = useState("");
  const [imageCID, setImageCID] = useState("");
  const [musicCID, setMusicCID] = useState("");
  const [metaCID, setMetaCID] = useState("");
  const [proposalId, setProposalId] = useState(null);
  const [inProgressNfts, setInProgressNfts] = useState([]);
  const [inProgressID, setInProgressID] = useState(null);
  const [message, setMessage] = useState("");
  const [readyForVote, setReadyForVote] = useState(false);

  async function saveData() {
    const updatedNftDetails = {
      contract: contract,
      title: title,
      description: desc,
      image_cid: imageCID,
      music_cid: musicCID,
      meta_cid: metaCID
    }
    
    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    localStorage.setItem("in_progress_id", inProgressID);
    
    const returnedProposalID = await updateNft(inProgressID, updatedNftDetails);
    localStorage.setItem("last_proposal_id", returnedProposalID);
    setProposalId(returnedProposalID);
    setReadyForVote(true);
  }

  // Act on proposal for the Update Data
  async function finalizeUpdateNft() {
    const returnMessage = await actOnProposal(proposalId);
    setMessage(JSON.stringify(returnMessage));
  }

  // This useEffect is setting the proposalId, for finalizing
  useEffect(async () => {
    const index = localStorage.getItem("last_proposal_id") - 2;
    const savedId = localStorage.getItem("in_progress_id");
    const proposalList = await getListOfProposals(index);
    console.log("proposalList: ", proposalList);

    const inProgressProposals = proposalList.filter((proposalEntry) => proposalEntry.status === "InProgress");
    console.log("inProgressProposal: ", inProgressProposals);
    if (inProgressProposals.length === 0) return;

    const updateNftKindProposals = inProgressProposals.filter((proposalEntry) => proposalEntry.kind.hasOwnProperty("UpdatePrepairedNft"));
    console.log("updateNftKindProposals: ", updateNftKindProposals);

    const i = updateNftKindProposals.findIndex((proposalEntry) => proposalEntry.kind.UpdatePrepairedNft.id === parseInt(savedId));
    console.log("the Entry: ", updateNftKindProposals[i]);

    const theId = updateNftKindProposals[i].id;
    setProposalId(theId);

  }, []);

  useEffect(async () => {
    const list = await getListOfAllInProgressNfts();
    setInProgressNfts(list);
  }, []);

  function setPreviouslySavedValues(id) {
    const index = inProgressNfts.findIndex((data) => data.id === id);
    
    setContract(inProgressNfts[index].contract);
    setTitle(inProgressNfts[index].title);
    setDesc(inProgressNfts[index].desc);
    setImageCID(inProgressNfts[index].image);
    setMusicCID(inProgressNfts[index].music);
    setMetaCID(inProgressNfts[index].meta);
    setInProgressID(id);
  }


  return (
    <>
      <Nav />
      <main>
        <h2>Updating Data</h2>

        <section>
          <p>{"Here we would need to find the previous entry with the unique ID, and append/overwrite it. We need to be sure that we only offer changing the fields for that prepaired NFT, that is by the same artist."}</p>
        </section>

        <section>
          <ul id="inProgressList">
            <p>{"These are the in progress NFTS (as JSON objects). Click on an element to edit."}</p>
            {inProgressNfts.map((inProgressNft) => {
              const stringifiedNft = JSON.stringify(inProgressNft, null, "\t");
              return (
                <li key={inProgressNft.id} onClick={() => setPreviouslySavedValues(inProgressNft.id)} className="inProgressListElement">
                  <br></br>
                  <code>{stringifiedNft}</code>
                  <br></br>
                </li>
              );
            })}
          </ul>
        </section>

        <section>
          <p>{"Current values: "}</p>
          <label>Minting Contract</label>
          <input className="cidInput" value={contract} onChange={(e) => setContract(e.target.value)}></input>{"not sure if we should allow changing this or not."}<br></br>

          <label>Title</label>
          <input className="cidInput" value={title} onChange={(e) => setTitle(e.target.value)}></input><br></br>

          <label>Description</label>
          <input className="cidInput" value={desc} onChange={(e) => setDesc(e.target.value)}></input><br></br>

          <label>Image CID</label>
          <input className="cidInput" value={imageCID} onChange={(e) => setImageCID(e.target.value)}></input><br></br>

          <label>Music Folder CID</label>
          <input className="cidInput" value={musicCID} onChange={(e) => setMusicCID(e.target.value)}></input><br></br>

          <label>Meta JSON CID</label>
          <input className="cidInput" value={metaCID} onChange={(e) => setMetaCID(e.target.value)}></input><br></br>
          
          <button onClick={saveData}>Save Data</button>
        </section>


        <section>
          {readyForVote && <p className="finalizeMessage">You can click on finalize now!</p>}
          <p>{"We will need to act on the proposal that we've just created. For that, we need the "}<code>{"last_proposal_id"}</code>{", and we need to get the proposals from that index, or from before that index, let's say 10."}</p>
          <p>{"We need to find that proposal that we've just created."}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"We are looking for a proposal that is "}<code>{"InProgress"}</code>{", and the title is the NFT title that we've just set."}</p>
          <p>{"And proposal kind is "}<code>{"UpdatePrepairedNft"}</code></p>
          <p>{"We save the contract name to LocalStorage as well."}</p>
          <p>{". "}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finalizeUpdateNft}>Finalize Prepair NFT</button>
          <p><code>{message}</code></p>
        </section>
      </main>
    </>
  )
}
