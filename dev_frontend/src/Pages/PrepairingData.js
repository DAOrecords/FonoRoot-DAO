import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, getLastProposalId, getListOfAllInProgressNfts, getListOfProposals, prepairNft } from '../utils';


export default function TestPageTwo() {
  const [contract, setContract] = useState("nft.soundsplash.testnet");
  const [title, setTitle] = useState("Smiling Sun");
  const [desc, setDesc] = useState("NFT about a Smiling Sun");
  const [imageCID, setImageCID] = useState("QmerincKVRPTXh1z41725mFNvGp31UBgfyms5xWi1taNuQ");
  const [musicCID, setMusicCID] = useState("QmU51uX3B44Z4pH2XimaJ6eScRgAzG4XUrKfsz1yWVCo6f");
  const [metaCID, setMetaCID] = useState("QmeCBSWQcDwn3ktKdEfDn68bt2PQbx3khyqGQAeYAVUHpb");
  const [proposalId, setProposalId] = useState(null);
  const [inProgressNfts, setInProgressNfts] = useState([]);
  const [message, setMessage] = useState("");

  async function saveData() {
    const newNftDetails = {
      contract: contract,
      title: title,
      description: desc,
      image_cid: imageCID,
      music_cid: musicCID,
      meta_cid: metaCID
    }

    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    localStorage.setItem("nft_title_when_prepairing", title);

    prepairNft(newNftDetails);
    // save ID somehow
  }

  // Act on proposal for the Creation of New Group
  async function finalizePrepairNft() {
    const returnMessage = await actOnProposal(proposalId);
    setMessage(returnMessage);
  }

  // This useEffect is setting the proposalId, for finalizing
  useEffect(async () => {
    const index = localStorage.getItem("last_proposal_id") - 2;
    const savedNftTitle = localStorage.getItem("nft_title_when_prepairing");
    const proposalList = await getListOfProposals(index);
    console.log("proposalList: ", proposalList);

    const inProgressProposals = proposalList.filter((proposalEntry) => proposalEntry.status === "InProgress");
    console.log("inProgressProposal: ", inProgressProposals);
    if (inProgressProposals.length === 0) return;

    const prepairNftKindProposals = inProgressProposals.filter((proposalEntry) => proposalEntry.kind.hasOwnProperty("PrepairNft"));
    console.log("prepairNftKindProposals: ", prepairNftKindProposals);

    const i = prepairNftKindProposals.findIndex((proposalEntry) => proposalEntry.kind.PrepairNft.nft_data.title === savedNftTitle);
    console.log("the Entry: ", prepairNftKindProposals[i]);

    const theId = prepairNftKindProposals[i].id;
    setProposalId(theId);

  }, []);

  useEffect(async () => {
    const list = await getListOfAllInProgressNfts();
    setInProgressNfts(list);
  }, []);


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
          
          <label>Minting Contract</label>
          <input className="cidInput" value={contract} onChange={(e) => setContract(e.target.value)}></input><br></br>

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
          <p>{"We will need to act on the proposal that we've just created. For that, we need the "}<code>{"last_proposal_id"}</code>{", and we need to get the proposals from that index, or from before that index, let's say 10."}</p>
          <p>{"We need to find that proposal that we've just created."}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"We are looking for a proposal that is "}<code>{"InProgress"}</code>{", and the title is the NFT title that we set."}</p>
          <p>{"And proposal kind is "}<code>{"PrepairNft"}</code></p>
          <p>{"We save the nft title to LocalStorage as well."}</p>
          <p>{"---"}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finalizePrepairNft}>Finalize Prepair NFT</button>
          <p><code>{message}</code></p>
        </section>

        <section>
          <ul id="inProgressList">
            <p>{"These are the in progress NFTS (as JSON objects):"}</p>
            {inProgressNfts.map((inProgressNft) => {
              const stringifiedNft = JSON.stringify(inProgressNft, null, "\t");
              return (
                <li key={inProgressNft.id} >
                  <br></br><code>{stringifiedNft}</code><br></br>
                </li>
              );
            })}
          </ul>
        </section>
      </main>
    </>
  )
}
