// scripts/updater.mjs

import fetch from 'node-fetch';
import { getOctokit, context } from '@actions/github';
import fs from 'fs';

import updatelog from './updatelog.mjs';

const token = process.env.GITHUB_TOKEN;

const platformMap = {
  "darwin-aarch64": "macOS",
  // "darwin": "macOS",
  "darwin-x86_64": "macOS",
  "windows-x86_64": "Windows",
  // "windows-x86_64-nsis": "Windows",
  // "windows-x86_64-msi": "Windows"
};

const targetMap = {
  "darwin-aarch64": "aarch64",
  // "darwin": "universal",
  "darwin-x86_64": "x86_64",
  "windows-x86_64": "x86_64-nsis",
  // "windows-x86_64-nsis": "x86_64-nsis",
  // "windows-x86_64-msi": "x86_64-msi"
};

const installerMap = {
  "darwin-aarch64": "dmg",
  // "darwin": "dmg",
  "darwin-x86_64": "dmg",
  "windows-x86_64": "exe",
  // "windows-x86_64-nsis": "exe",
  // "windows-x86_64-msi": "msi"
};

// 需要生成的静态 json 文件数据，根据自己的需要进行调整
const updateData = {
  version: '',
  // 使用 UPDATE_LOG.md，如果不需要版本更新日志，则将此字段置空
  notes: '',
  pub_date: new Date().toISOString(),
  platforms: {
    // win64: { signature: '', url: '' }, // compatible with older formats
    // linux: { signature: '', url: '' }, // compatible with older formats
    // darwin: { signature: '', url: '' }, // compatible with older formats
    'darwin-aarch64': { signature: '', url: '' },
    'darwin-x86_64': { signature: '', url: '' },
    // 'linux-x86_64': { signature: '', url: '' },
    'windows-x86_64': { signature: '', url: '' },
    // 'windows-x86_64-nsis': { signature: '', url: '' },
    // 'windows-x86_64-msi': { signature: '', url: '' },
    // 'windows-i686': { signature: '', url: '' }, // no supported
  },
};

async function updater() {
  if (!token) {
    console.log('GITHUB_TOKEN is required');
    process.exit(1);
  }

  // 用户名，仓库名
  const options = { owner: context.repo.owner, repo: context.repo.repo };
  const github = getOctokit(token);

  // 获取 tag
  const { data: tags } = await github.rest.repos.listTags({
    ...options,
    per_page: 10,
    page: 1,
  });

  // 过滤包含 `v` 版本信息的 tag
  const tag = tags.find((t) => t.name.startsWith('v'));
  // console.log(`${JSON.stringify(tag, null, 2)}`);

  if (!tag) return;

  // 获取此 tag 的详细信息
  const { data: latestRelease } = await github.rest.repos.getReleaseByTag({
    ...options,
    tag: tag.name,
  });
  console.log('latestRelease: ', latestRelease);

  updateData.version = tag.name;
  updateData.notes = updatelog(tag.name);

  const setSig = async (asset) => {
    // 是签名
    if (/.sig$/.test(asset.name)) {
      let sig = await getSignature(asset.browser_download_url);
      console.log('[setSig] get sig: ', sig);
      // macos
      if (/_aarch64.app.tar.gz.sig$/.test(asset.name)) {
        updateData.platforms['darwin-aarch64'].signature = sig;
      }
      // else if (/_universal.app.tar.gz.sig$/.test(asset.name)) {
      //   updateData.platforms['darwin'].signature = sig;
      // } 
      else if (/_x64.app.tar.gz.sig$/.test(asset.name)) {
        updateData.platforms['darwin-x86_64'].signature = sig;
      }
      // windows
      else if (/_x64-setup.nsis.zip.sig$/.test(asset.name)) {
        // updateData.platforms['windows-x86_64-nsis'].signature = sig;
        updateData.platforms['windows-x86_64'].signature = sig;
      }
      // else if (/_x64_en-US.msi.zip.sig$/.test(asset.name)) {
      //   updateData.platforms['windows-x86_64-msi'].signature = sig;
      // }


    }
  }

  const setUrl = async (asset) => {
    // 设置下载链接
    // macos
    if (/_aarch64.app.tar.gz$/.test(asset.name)) {
      updateData.platforms['darwin-aarch64'].url = asset.browser_download_url;
    }
    // else if (/_universal.app.tar.gz$/.test(asset.name)) {
    //   updateData.platforms['darwin'].url = asset.browser_download_url;
    // } 
    else if (/_x64.app.tar.gz$/.test(asset.name)) {
      updateData.platforms['darwin-x86_64'].url = asset.browser_download_url;
    }
    // windows
    else if (/_x64-setup.nsis.zip$/.test(asset.name)) {
      updateData.platforms['windows-x86_64'].url = asset.browser_download_url;
      // updateData.platforms['windows-x86_64-nsis'].url = asset.browser_download_url;
    }
    // else if (/_x64_en-US.msi.zip$/.test(asset.name)) {
    //   updateData.platforms['windows-x86_64-msi'].url = asset.browser_download_url;
    // }
    console.log('[setAsset] set sig, updateData: ', updateData);

  }

  const setAsset = async (asset) => {
    console.log('[setAsset] asset: ', asset);
    console.log('[setAsset] name: ', asset.name);
    console.log('[setAsset] browser_download_url: ', asset.browser_download_url);
    // console.log('[setAsset] reg: ', reg);
    await setSig(asset);
    await setUrl(asset);
  };


  const promises = latestRelease.assets.map(async (asset) => {
    // const platformsToCheck = {
    //   '.msi.zip': ['win64', 'windows-x86_64'],
    //   '.app.tar.gz': ['darwin', 'darwin-x86_64', 'darwin-aarch64'],
    //   // '.dmg': ['darwin', 'darwin-x86_64', 'darwin-aarch64'],
    //   // '.app.tar.gz.sig': ['darwin', 'darwin-x86_64', 'darwin-aarch64'],
    //   // '.tar.gz': ['linux', 'linux-x86_64']
    // };

    await setAsset(asset);
    // for (const [pattern, platforms] of Object.entries(platformsToCheck)) {
    //   // await _setAsset(asset, new RegExp(pattern), platforms);
    // };
  })

  await Promise.allSettled(promises);

  if (!fs.existsSync('updater')) {
    fs.mkdirSync('updater');
  }

  // 将数据写入文件
  fs.writeFileSync(
    './updater/install.json',
    JSON.stringify(updateData, null, 2)
  );
  console.log('Generate updater/install.json');
}

updater().catch(console.error);
addPackageVersion().catch(console.error)

// 获取签名内容
async function getSignature(url) {
  try {
    const response = await fetch(url, {
      method: 'GET',
      headers: { 'Content-Type': 'application/octet-stream' },
    });
    return response.text();
  } catch (_) {
    return '';
  }
}


// 封装发送POST请求的函数
async function addPackageVersion() {
  const url = 'https://boss.ffdev.cc/v1/version';
  for (const [platform, data] of Object.entries(updateData.platforms)) {



    const packageData = {
      platform: platformMap[platform] || "Unknown",
      version: updateData.version,
      target: targetMap[platform] || "Unknown",
      installer: installerMap[platform] || "Unknown",
      note: updateData.notes,
      download_url: data.url,
    };

    const requestOptions = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
        // 如果有需要的话，可以添加其他header
      },
      body: JSON.stringify(packageData)
    };

    try {
      const response = await fetch(url, requestOptions);
      if (response.ok) {
        console.log('Added package version information successfully!');
      } else {
        console.error('Failed to add package version information:', response.statusText);
      }
    } catch (error) {
      console.error('An error has occurred:', error);
    }
  };
}