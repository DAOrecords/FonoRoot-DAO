import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, getLastProposalId, getListOfAllInProgressNfts, getListOfProposals, mintNft } from '../utils';


export default function TestPageFour() {
  const [inProgressNfts, setInProgressNfts] = useState([]);
  const [selected, setSelected] = useState(null);
  const [ready, setReady] = useState(false);
  const [message, setMessage] = useState("");
  const [proposalId, setProposalId] = useState(null);

  useEffect(async () => {
    const list = await getListOfAllInProgressNfts();
    setInProgressNfts(list);
  }, []);

  useEffect(() => {
    if (selected === null) {
      setReady(false);
      return;
    }

    const isReady = (
      inProgressNfts[selected].title &&
      inProgressNfts[selected].desc &&
      inProgressNfts[selected].contract &&
      inProgressNfts[selected].image &&
      inProgressNfts[selected].meta &&
      inProgressNfts[selected].music
    );

    setReady(isReady);

  }, [selected]);

  async function initiateMinting() {
    if (!ready) {
      window.alert("This NFT can't be minted! Not all the necesarry info is supplied. Please fill in all the fields first!");
      return;
    }

    const selectedForMinting = inProgressNfts[selected];
    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    localStorage.setItem("in_progress_id", selectedForMinting.id);

    mintNft(selectedForMinting.id);
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

    const mintNftKindProposals = inProgressProposals.filter((proposalEntry) => proposalEntry.kind.hasOwnProperty("MintRoot"));
    console.log("mintNftKindProposals: ", mintNftKindProposals);

    const i = mintNftKindProposals.findIndex((proposalEntry) => proposalEntry.kind.MintRoot.id === parseInt(savedId));
    console.log("the Entry: ", mintNftKindProposals[i]);

    const theId = mintNftKindProposals[i].id;
    setProposalId(theId);

  }, []);

  // Act on proposal for the Minting
  async function finalizeUpdateNft() {
    console.log(proposalId)
    const returnMessage = await actOnProposal(proposalId);
    setMessage(JSON.stringify(returnMessage));
  }  


  return (
    <>
      <Nav />
      <main>
        <h2>Minting</h2>

        <section>
          <ul id="inProgressList">
            <p>{"These are the in progress NFTS (as JSON objects). Click on an element to edit."}</p>
            {inProgressNfts.map((inProgressNft) => {
              const stringifiedNft = JSON.stringify(inProgressNft, null, "\t");
              return (
                <li 
                  key={inProgressNft.id} 
                  onClick={() => setSelected(inProgressNfts.findIndex((cur) => cur.id === inProgressNft.id))} 
                  className="inProgressListElement"
                >
                  <br></br>
                  <code>{stringifiedNft}</code>
                  <br></br>
                </li>
              );
            })}
          </ul>
        </section>

        {(selected !== null) && <section style={{ background: ready ? "#00FF00" : "#FF0000" }}>
          <h3>{"Selected for Minting: "}</h3>
          <p><strong>{"Title: "}</strong>{inProgressNfts[selected].title}</p>
          <p><strong>{"Description: "}</strong>{inProgressNfts[selected].desc}</p>
          <p><strong>{"Contract: "}</strong>{inProgressNfts[selected].contract}</p>
          <p><strong>{"Image: "}</strong>{inProgressNfts[selected].image}</p>
          <p><strong>{"Meta: "}</strong>{inProgressNfts[selected].meta}</p>
          <p><strong>{"Music: "}</strong>{inProgressNfts[selected].music}</p>
        </section>}

        <section>
          <p>{"The Artist can initiate the minting of the NFT"}</p>
          <button onClick={initiateMinting}>Mint Selected NFT Now</button>
        </section>

        <section>
          <p>{"We will need to act on the proposal that we've just created. See Registration"}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finalizeUpdateNft}>Finalize Prepair NFT</button>
          <p><code>{message}</code></p>
        </section>

        <section>
          <p>{"Or, the Artist can schedule the minting of the NFT (through CronCat or othe means)"}</p>
        </section>
      </main>
    </>
  )
}
